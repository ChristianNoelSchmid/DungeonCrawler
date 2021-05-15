using System.Collections;
using System.Collections.Generic;

using UnityEngine;

using DungeonCrawler.Models;
using DungeonCrawler.Monobehaviours;

namespace DungeonCrawler.Monobehaviours
{
    public class ActorGenerator : MonoBehaviour
    {
        [SerializeField]
        private GameObject _playerTemplate;

        [SerializeField]
        private GameObject[] _monsterTemplates;

        private Dictionary<int, GridPosition> _actorPositions;

        private void Awake() =>
            _actorPositions = new Dictionary<int, GridPosition>();

        private readonly WaitForEndOfFrame _waitForEndOfFrame = new WaitForEndOfFrame();

        public void UpdatePosition(int id, PositionModel position)
        {
            // Because the server may not have sent the client the new
            // monster before sending a move update, it must be checked
            if(_actorPositions.ContainsKey(id))
            {
                _actorPositions[id].FromPositionModel(position);
                Obstacles.UpdateObstacle(_actorPositions[id].transform, _actorPositions[id].Value);   
            }
        }

        public void SpawnPlayer(Player player)
        {
            GridPosition playerPos;
            _actorPositions.Add(
                player.Id,
                playerPos = Instantiate(_playerTemplate, new Vector3(-1000, 0), Quaternion.identity, null)
                    .GetComponent<GridPosition>()
            );

            UpdatePosition(player.Id, playerPos.ToPositionModel());
        }
        public void SpawnMonster(MonsterInstance monster)
        {
            (var templateId, var instanceId) = 
                (monster.TemplateId, monster.InstanceId);

            GridPosition monsterPosition;
            _actorPositions.Add(
                instanceId,
                monsterPosition = Instantiate(_monsterTemplates[templateId], new Vector3(-1000, 0), Quaternion.identity, null)
                    .GetComponent<GridPosition>()
            );

            UpdatePosition(instanceId, monsterPosition.ToPositionModel());
        }
        public void RemoveById(int id) 
        {
            if(_actorPositions.ContainsKey(id))
            {
                GameObject.Destroy(_actorPositions[id].gameObject);
                _actorPositions.Remove(id);
            }
        }

        public void HitOther(int attId, int defId) 
        {
            if(_actorPositions.ContainsKey(attId) && _actorPositions.ContainsKey(defId))
            {
                var dir = ((Vector2)(_actorPositions[attId].Value - _actorPositions[defId].Value)).normalized;
                switch (dir.x, dir.y)
                {
                    case (var x, var y) when y > 0.25f:
                        StartCoroutine(AttackAnim(attId, 1)); break;
                    case (var x, var y) when y < -0.25f:
                        StartCoroutine(AttackAnim(attId, 2)); break;
                    case (var x, var y) when x < -0.25f:
                        StartCoroutine(AttackAnim(attId, 3)); break;
                    default:
                        StartCoroutine(AttackAnim(attId, 4)); break;
                }
            }
        }

        private IEnumerator AttackAnim(int attId, int dir) 
        {
            var animator = _actorPositions[attId].GetComponent<Animator>();
            animator.SetInteger("attackDir", dir);
            yield return _waitForEndOfFrame; 

            animator.SetInteger("attackDir", 0);

        }

        public void MissOther(int attId, int defId) 
        {
            HitOther(attId, defId);
        }
    }
}