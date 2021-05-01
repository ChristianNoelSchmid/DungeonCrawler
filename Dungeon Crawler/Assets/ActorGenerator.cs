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
            (var id, var position) = (player.Id, player.Position);

            _actorPositions.Add(
                id,
                Instantiate(_playerTemplate, new Vector3(position.X, position.Y), Quaternion.identity, null)
                    .GetComponent<GridPosition>()
            );

            UpdatePosition(id, position);
        }
        public void SpawnMonster(MonsterInstance monster)
        {
            (var templateId, var instanceId, var position) = 
                (monster.TemplateId, monster.InstanceId, monster.Position);

            _actorPositions.Add(
                instanceId,
                Instantiate(_monsterTemplates[templateId], new Vector3(position.X, position.Y), Quaternion.identity, null)
                    .GetComponent<GridPosition>()
            );

            UpdatePosition(instanceId, position);
        }
    }
}