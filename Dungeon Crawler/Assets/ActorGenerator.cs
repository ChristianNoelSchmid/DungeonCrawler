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
        private Dictionary<int, ActorStatus> _actorStatuses;
        private HashSet<int> _playerIds;

        private int _clientPlayerId;
        public int ClientPlayerId => _clientPlayerId;

        private void Awake()
        {
            _actorPositions = new Dictionary<int, GridPosition>();
            _playerNames = new Dictionary<int, string>();
            _actorStatuses = new Dictionary<int, ActorStatus>();
            _playerIds = new HashSet<int>();
        }

        private readonly WaitForEndOfFrame _waitForEndOfFrame = new WaitForEndOfFrame();

        public void UpdatePosition(int id, TransformModel transform)
        {
            // Because the server may not have sent the client the new
            // monster before sending a move update, it must be checked
            if (_actorPositions.ContainsKey(id))
            {
                _actorPositions[id].FromTransformModel(transform);
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
            _actorStatuses[id] = position.GetComponent<ActorStatus>();
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
            _actorStatuses[player.Id] = playerPos.GetComponent<ActorStatus>();
            _actorStatuses[player.Id].Status = player.Status;
            _playerIds.Add(player.Id);

            UpdatePosition(player.Id, playerPos.ToTransformModel());
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

            _actorStatuses[instanceId] = monsterPosition.GetComponent<ActorStatus>();
            _actorStatuses[instanceId].Status = Status.Active;

            UpdatePosition(instanceId, monsterPosition.ToTransformModel());
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

        public void MissOther(int attId, int defId) => HitOther(attId, defId);
        public void AttackTowards(int attId, Vector2Int pos)
        {
           var dir = _actorPositions[attId].Value - pos;
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

        public void KillActor(int id)
        {
            if (_actorStatuses.TryGetValue(id, out var status))
            {
                status.Status = Status.Dead;
                Obstacles.RemoveObstacle(status.transform);
            }
        }

        public void EscapeActor(int id)
        {
            if (_actorStatuses.TryGetValue(id, out var status))
            {
                status.Status = Status.Escaped;
                Obstacles.RemoveObstacle(status.transform);
            }
        }

        public ReadOnlyCollection<Tuple<ActorStatus, string>> GetPlayers()
        {
            return _playerIds
                .Select(id => Tuple.Create(_actorStatuses[id], _playerNames[id]))
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
            _actorStatuses.Clear();
            _playerNames.Clear();
            _playerIds.Clear();
        }

        public void ChargeAttack(int id)
        {
            var animator = _actorPositions[id].GetComponent<Animator>();
            animator.SetTrigger("Charging");
        }

        public int NonPlayerAt(Vector2Int pos)
        {
            if(_actorPositions.Any(a => a.Value.Value == pos))
            {
                var defdId = _actorPositions.Single(a => a.Value.Value == pos).Key;
                if(!_playerIds.Contains(defdId))
                    return defdId;
            }
            return -1;
        }
    }
}