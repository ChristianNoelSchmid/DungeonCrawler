using System.Collections;
using System.Collections.Generic;
using System.Linq;

using UnityEngine;

using DungeonCrawler.Models;
using DungeonCrawler.Networking;
using Assets.Scripts;
using DungeonCrawler.Networking.NetworkEvents;

namespace DungeonCrawler.Monobehaviours
{    
    internal struct MoveRepeatTimer {
        public KeyCode code;
        public float timer;
        public Vector2Int direction;
    }

    public class PlayerMovement : MonoBehaviour
    {
        private const float MOVE_INC = 0.125f;
        private const float DIAG_INC = 1.4f;

        public static bool Disabled { get; set; } = false;

        [SerializeField]
        private NetworkDatagramHandler _datagramHandler;

        [SerializeField]
        private ActorGenerator _actorGen;
        private static Camera _mainCamera;
        private GridPosition _gridPosition;
        private MoveRepeatTimer[] _timers;

        private HumanoidRenderer _renderer;
        private Animator _animator;

        private float _attackTimeout = 0.0f;

        private Transform _transform;
        private bool _secondaryHeld = false;
        public static float Angle(Vector2 pos1, Vector2 pos2) =>
        Mathf.Atan2(pos2.y - pos1.y, pos2.x - pos1.x);

        private void Awake()
        {
            _mainCamera = Camera.main;
            _gridPosition = GetComponent<GridPosition>();
            _transform = transform;
            _animator = GetComponent<Animator>();
            _renderer = GetComponent<HumanoidRenderer>(); 
            if (_gridPosition == null)
                Debug.LogError("Expected GridPosition on MonoBehaviour, but it wasn't found.");

            _timers = new MoveRepeatTimer[]
            {
                new MoveRepeatTimer { code = KeyCode.A, timer = 0, direction = new Vector2Int(-1, 0) },
                new MoveRepeatTimer { code = KeyCode.D, timer = 0, direction = new Vector2Int(1, 0) },
                new MoveRepeatTimer { code = KeyCode.W, timer = 0, direction = new Vector2Int(0, 1) },
                new MoveRepeatTimer { code = KeyCode.S, timer = 0, direction = new Vector2Int(0, -1) },
            };
        }

        private void Update()
        {
            if (Disabled) return;

            int keysDown = _timers.Where(t => Input.GetKey(t.code)).ToArray().Length;
            var newPos = Vector2Int.zero;

            for (int i = 0; i < _timers.Length; ++i)
            {
                if (Input.GetKeyDown(_timers[i].code))
                    newPos += _timers[i].direction;
                else if (Input.GetKey(_timers[i].code))
                {
                    _timers[i].timer += Time.deltaTime
                        / (Input.GetKey(KeyCode.LeftShift) ? 4.0f : 1.0f);
                    if (_timers[i].timer >= (keysDown <= 1 ? MOVE_INC : (MOVE_INC * DIAG_INC)))
                    {
                        newPos += _timers[i].direction;
                        _timers[i].timer = 0.0f;
                    }
                }
                else
                    _timers[i].timer = 0.0f;
            }

            if (Input.GetKeyDown(KeyCode.Space))
                GetComponent<ActorLight>().Enabled = !GetComponent<ActorLight>().Enabled;

            if (newPos != Vector2Int.zero)
                _animator.SetTrigger("Move");

            if (Obstacles.UpdateObstacle(_transform, _gridPosition.Value + newPos))
                _gridPosition.Value += newPos;

            var mouseWorldPos = MousePosition;
            if (mouseWorldPos.x > _transform.position.x)
                _transform.localScale = Vector3.one;
            else
                _transform.localScale = new Vector3(-1, 1, 1);

            HandleCombat();
        }

        private void LateUpdate()
        {
            _animator.SetInteger("AttackDirection", 0);
        }

        public void HandleCombat()
        {
            if(_attackTimeout > 0.0f) _attackTimeout -= Time.deltaTime;
            if (Input.GetMouseButton(0) && _attackTimeout <= 0.0f)
            {
                var angle = Angle(_transform.position, MousePosition);
                int dir = 4;
                Vector2Int attackPos = _gridPosition.Value + new Vector2Int(MousePosition.x < _transform.position.x ? -1 : 1, 0);

                if (angle > Mathf.PI / 3.0f && angle < 2.0f * Mathf.PI / 3.0f)
                {
                    dir = 1;
                    attackPos = _gridPosition.Value + new Vector2Int(0, 1);
                }
                else if (angle > -2.0f * Mathf.PI / 3.0f && angle < -Mathf.PI / 3.0f)
                {
                    dir = 2;
                    attackPos = _gridPosition.Value + new Vector2Int(0, -1);
                }

                _animator.SetInteger("AttackDirection", dir);
                _renderer.TriggerAction(ActionType.PrimaryPressed);

                int enemyId = _actorGen.NonPlayerAt(attackPos);
                if(enemyId != -1)
                    _datagramHandler.SendDatagram(new HitAttempt
                    {
                        Model = new DataModel<MissModel>
                        {
                            Id = _actorGen.ClientPlayerId,
                            Value = new MissModel { DefenderId = enemyId }
                        }
                    }.CreateString(), false);

                _attackTimeout = 0.35f;
            }

            if (Input.GetMouseButton(1))
            {
                _renderer.TriggerAction(ActionType.SecondaryHeld);
                _secondaryHeld = true;
            }
            else if (_secondaryHeld)
            {
                _renderer.TriggerAction(ActionType.SecondaryReleased);
                _secondaryHeld = false;
            }
        }

        public static Vector2 MousePosition => _mainCamera.ScreenToWorldPoint(Input.mousePosition);
    }
}