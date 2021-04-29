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

        private Dictionary<int, GridPosition> _playerPositions;

        private void Awake() =>
            _playerPositions = new Dictionary<int, GridPosition>();

        public void UpdatePlayerPosition(int id, Position position)
        {
            if(!_playerPositions.ContainsKey(id))
            {
                _playerPositions.Add(
                    id,
                    Instantiate(_playerTemplate, new Vector3(position.X, position.Y), Quaternion.identity, null)
                        .GetComponent<GridPosition>()
                );
            }
            (_playerPositions[id].X, _playerPositions[id].Y) = (position.X, position.Y);
            Obstacles.UpdateObstacle(_playerPositions[id].transform, new Vector2Int(position.X, position.Y));
        }
    }
}