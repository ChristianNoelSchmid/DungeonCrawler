using System.Collections;
using System.Collections.Generic;
using System.Linq;

using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Monobehaviours
{
    internal struct MoveRepeatTimer {
        public KeyCode code;
        public float timer;
        public Vector2Int direction;
    }

    public class PlayerMovement : MonoBehaviour
    { 
        private GridPosition _gridPosition;
        private MoveRepeatTimer [] _timers;

        private Transform _transform;

        private void Awake()
        {
            _gridPosition = GetComponent<GridPosition>();
            _transform = transform;
            if(_gridPosition == null)
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
            int keysDown = _timers.Where(t => Input.GetKey(t.code)).ToArray().Length;
            var newPos = Vector2Int.zero;
            
            for(int i = 0; i < _timers.Length; ++i) 
            {
                if(Input.GetKeyDown(_timers[i].code))        
                    newPos += _timers[i].direction;
                else if(Input.GetKey(_timers[i].code)) 
                {
                    _timers[i].timer += Time.deltaTime
                        / (Input.GetKey(KeyCode.LeftShift) ? 4.0f : 1.0f);
                    if(_timers[i].timer >= (keysDown <= 1 ? 0.1f : 0.14f)) 
                    {
                        newPos += _timers[i].direction;
                        _timers[i].timer = 0.0f;
                    }
                }
                else 
                    _timers[i].timer = 0.0f;
            }

            if(Obstacles.UpdateObstacle(_transform, _gridPosition.Value + newPos))
            {
                _gridPosition.Value += newPos;
                if(newPos.x > 0) _gridPosition.Direction = Direction.Right;
                else if(newPos.x < 0) _gridPosition.Direction = Direction.Left;
            }
        }
    }
}