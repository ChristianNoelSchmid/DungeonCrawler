using System.Collections;
using System.Collections.Generic;

using UnityEngine;

using DungeonCrawler.Models;
using System.Collections.ObjectModel;
using System;
using System.Linq;
using Assets.Scripts.Models;

namespace DungeonCrawler.Monobehaviours
{
    public class ActorGenerator : MonoBehaviour
    {
        [SerializeField]
        private GameObject _playerTemplate;

        [SerializeField]
        private GameObject[] _monsterTemplates;

        private Dictionary<int, GridPosition> _actorPositions;
        private Dictionary<int, string> _playerNames;
        private Dictionary<int, ActorStatus> _playerStatuses;
        private HashSet<int> _playerIds;

        private int _clientPlayerId;

        private void Awake()
        {
            _actorPositions = new Dictionary<int, GridPosition>();
            _playerNames = new Dictionary<int, string>();
            _playerStatuses = new Dictionary<int, ActorStatus>();
            _playerIds = new HashSet<int>();
        }

        private readonly WaitForEndOfFrame _waitForEndOfFrame = new WaitForEndOfFrame();

        public void UpdatePosition(int id, PositionModel position)
        {
            // Because the server may not have sent the client the new
            // monster before sending a move update, it must be checked
            if (_actorPositions.ContainsKey(id))
            {
                _actorPositions[id].FromPositionModel(position);
                if(Obstacles.UpdateObstacle(_actorPositions[id].transform, _actorPositions[id].Value))
                {
                    _actorPositions[id].GetComponent<Animator>().SetTrigger("Move");
                }
            }
        }
        public void AddClientPlayer(GridPosition position, string name, int id)
        { 
            _clientPlayerId = id;
            _actorPositions[id] = position;
            _playerNames[id] = name;
            _playerStatuses[id] = position.GetComponent<ActorStatus>();
            _playerIds.Add(id);
        }

        public void SpawnPlayer(Player player)
        {
            GridPosition playerPos;
            _actorPositions.Add(
                player.Id,
                playerPos = Instantiate(_playerTemplate, new Vector3(-1000, 0), Quaternion.identity, null)
                    .GetComponent<GridPosition>()
            );
            _playerNames[player.Id] = player.Name;
            _playerStatuses[player.Id] = playerPos.GetComponent<ActorStatus>();
            _playerStatuses[player.Id].Status = player.Status;
            _playerIds.Add(player.Id);

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
            if (_actorPositions.ContainsKey(id))
            {
                Destroy(_actorPositions[id].gameObject);
                _actorPositions.Remove(id);
                _playerNames.Remove(id);
            }
        }

        public void HitOther(int attId, int defId)
        {
            if (_actorPositions.ContainsKey(attId) && _actorPositions.ContainsKey(defId))
            {
                var dir = _actorPositions[attId].Value - _actorPositions[defId].Value;
                StartCoroutine(AttackAnim(
                    attId,
                    (dir.x, dir.y) switch
                    {
                        (0, -1) => 1,
                        (0, 1) => 2,
                        _ => 4,
                    }
                ));
            }
        }

        private IEnumerator AttackAnim(int attId, int dir)
        {
            var animator = _actorPositions[attId].GetComponent<Animator>();
            animator.SetInteger("AttackDirection", dir);
            yield return _waitForEndOfFrame;

            animator.SetInteger("AttackDirection", 0);

        }

        public void MissOther(int attId, int defId)
        {
            HitOther(attId, defId);
        }

        public void KillActor(int id)
        {
            if (_playerStatuses.TryGetValue(id, out var status))
            {
                status.Status = Status.Dead;
                Obstacles.RemoveObstacle(status.transform);
            }
        }

        public void EscapeActor(int id)
        {
            if (_playerStatuses.TryGetValue(id, out var status))
            {
                status.Status = Status.Escaped;
                Obstacles.RemoveObstacle(status.transform);
            }
        }

        public ReadOnlyCollection<Tuple<ActorStatus, string>> GetPlayers()
        {
            return _playerIds
                .Select(id => Tuple.Create(_playerStatuses[id], _playerNames[id]))
                .ToList()
                .AsReadOnly();
        }

        public void ResetAll()
        {
            var keys = _actorPositions.Keys.ToList();
            foreach (int key in keys) 
            { 
                Obstacles.RemoveObstacle(_actorPositions[key].transform);
                if(key != _clientPlayerId)
                    Destroy(_actorPositions[key].gameObject);
            }

            _actorPositions.Clear();
            _playerStatuses.Clear();
            _playerNames.Clear();
            _playerIds.Clear();
        }
    }
}