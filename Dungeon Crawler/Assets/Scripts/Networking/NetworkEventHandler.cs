using System;
using System.Collections;
using System.Collections.Generic;

using UnityEngine;

using DungeonCrawler.Models;
using DungeonCrawler.Networking.NetworkEvents;
using DungeonCrawler.Monobehaviours;
using System.Threading;
using Assets.Scripts.Networking.NetworkEvents;

namespace DungeonCrawler.Networking
{
    /// <summary>
    /// Handles the transfer of all Network events to their
    /// associated processes.
    /// </summary>
    public class NetworkEventHandler : MonoBehaviour
    {
        private NetworkDatagramHandler _datagramHandler;
        private WaitForSeconds _waitForInterval;

        private bool _networkingEnabled = false;

        [SerializeField]
        private DungeonGenerator _dungeonGen;

        [SerializeField]
        private ActorGenerator _actorGen;

        [SerializeField]
        private GridPosition _playerPosition;

        [SerializeField]
        private UIWatchPane _watchPane;
        [SerializeField]
        private UIResultsPane _resultsPane;
        [SerializeField]
        private UIStatBar _statBar;

        [SerializeField]
        private CameraMovement _cameraMovement;

        private Vector2Int _prevPlayerPosition;

        private const float _playerUpdateIntevalSeconds = 0.1f;

        private int _playerId;
        private string _playerName;
        private Queue<DatagramCallback> _callbackQueue;

        private Thread _pingThread;

        private void Awake()
        {
            _waitForInterval = new WaitForSeconds(_playerUpdateIntevalSeconds);
            _datagramHandler = GetComponent<NetworkDatagramHandler>();

            // Set the datagram handler given, upon recieving a message,
            // to send it to this NetworkEventHandler for parsing
            _datagramHandler.MessageRecieved += (_, callback) =>
                _callbackQueue.Enqueue(callback);

            _callbackQueue = new Queue<DatagramCallback>();
        }

        /// <summary>
        /// Begins the handler, sending the first PlayerJoined message
        /// </summary>
        public void StartHandler(string name)
        {
            _playerName = name;
            _networkingEnabled = true;
            _datagramHandler.SendDatagram(
                new Hello { Player = new Player { Name = _playerName } }.CreateString(),
                true
            );
            _pingThread = new Thread(BeginPinging) { IsBackground = true };
            _pingThread.Start();
            StartCoroutine(BeginUpdatingMovement());
        }

        private void Update()
        {
            if(!_networkingEnabled) return;

            // If there is an event in the queue, call it
            while(_callbackQueue.Count > 0)
                TransferEvent(_callbackQueue.Dequeue());
        }

        private void BeginPinging()
        {
            while(true)
            {
                _datagramHandler.SendDatagram(
                    Datagrams.Ping.CreateString(),
                    false
                );
                Thread.Sleep(1000);
            }
        }

        // Periodically sends ping information to the Server, 
        // with the local Player's GridPosition
        private IEnumerator BeginUpdatingMovement()
        {
            while(true)
            {
                if(_prevPlayerPosition != _playerPosition.Value)
                {
                    _datagramHandler.SendDatagram(
                        new Moved
                        {
                            Model = new DataModel<TransformModel>
                            {
                                Id = _playerId,
                                Value = _playerPosition.ToTransformModel()
                            }
                        }.CreateString(),
                        false
                    );
                    _prevPlayerPosition = _playerPosition.Value;
                }
                yield return _waitForInterval;
            }
        }

        /// <summary>
        /// Converts incoming text into an appropriate
        /// NetworkEvent.
        /// </summary>
        /// <param name="text">The string to parse</param>
        /// <returns>The NetworkEvent, with parsed data</returns>
        private NetworkEvent ParseEvent(string text)
        {
            var command = text.Split(new string [] { "::" }, 2, StringSplitOptions.None)[0];
            var args = text.Substring(command.Length + 2);
            Debug.Log(text);
            try 
            {
                return command switch
                {
                    "Welcome"    =>      new Welcome(args),
                    "NewPlayer"  =>      new NewPlayer(args),
                    "NewMonster" =>      new NewMonster(args),
                    "PlayerLeft" =>      new PlayerLeft(args),
                    "Charging"   =>      new Charging(args),
                    "AttkTowards"=>      new AttkTowards(args),
                    "Hit"        =>      new Hit(args),
                    "Miss"       =>      new Miss(args),
                    "Moved"      =>      new Moved(args),
                    "Dead"       =>      new Dead(args),
                    "Escaped"    =>      new Escaped(args),
                    "DungeonComplete" => new DungeonComplete(),
                    "Reconnect" =>       new Reconnect(),
                    _            =>      null,
                };
            }
            catch(Exception) { return null; }
        }

        /// <summary>
        /// Convert the parsed Event into an action, and return
        /// a datagram to the Server if applicable
        /// </summary>
        /// <param name="callback">The callback method for returning a datagram to the Server.</param>
        private void TransferEvent(DatagramCallback callback)
        {
            switch(ParseEvent(callback.Data))
            {
                case Welcome welcome:  // On Welcome, get the player Id, update the map with the StateSnapshot
                                       // info and begin the Player position transfer

                    _dungeonGen.Dungeon = welcome.Model.Value;
                    _cameraMovement.Target = _playerPosition.transform;
                    _playerPosition.Value = _dungeonGen.Dungeon.Entrance;
                    _playerId = welcome.Model.Id;

                    _actorGen.AddClientPlayer(_playerPosition, _playerName, _playerId);

                    PlayerMovement.Disabled = false;
                    _watchPane.SetVisible(false);
                    _resultsPane.SetVisible(false);
                    _statBar.SetHealth(10);
                    _playerPosition.GetComponent<ActorStatus>().Status = Assets.Scripts.Models.Status.Active;

                    break;
                
                case NewPlayer newPlayer:

                    _actorGen.SpawnPlayer(newPlayer.Model);
                    break;

                case NewMonster newMonster:
                    _actorGen.SpawnMonster(newMonster.Model);
                    break;

                case Moved moved:
                    if(moved.Model.Id == _playerId)
                        _playerPosition.FromTransformModel(moved.Model.Value);
                    else
                        _actorGen.UpdatePosition(moved.Model.Id, moved.Model.Value);
                    break;

                case PlayerLeft left: // On PlayerLeft, remove the Client's marker from the PlayerConnections
                    _actorGen.RemoveById(left.Model.Id); 
                    break;

                case Charging charging:
                    _actorGen.ChargeAttack(charging.Model.Id);
                    break;

                case Hit hit:
                    _actorGen.HitOther(hit.Model.Id, hit.Model.Value.DefenderId);
                    if(hit.Model.Value.DefenderId == _playerId)
                        _statBar.SetHealth(hit.Model.Value.HealthLeft);
                    break;

                case Miss miss:
                    _actorGen.MissOther(miss.Model.Id, miss.Model.Value.DefenderId);
                    break;

                case Dead dead:
                    _actorGen.KillActor(dead.Model.Id);
                    if (dead.Model.Id == _playerId)
                    {
                        PlayerMovement.Disabled = true;
                        _watchPane.OnStatusChange(true);
                        _watchPane.SetVisible(true);
                    }
                    break;

                case Escaped escaped:
                    _actorGen.EscapeActor(escaped.Model.Id);
                    if (escaped.Model.Id == _playerId)
                    {
                        PlayerMovement.Disabled = true;
                        _watchPane.OnStatusChange(false);
                        _watchPane.SetVisible(true);
                    }
                    break;

                case DungeonComplete _:
                    _watchPane.SetVisible(false);
                    _resultsPane.SetVisible(true);
                    break;

                case Reconnect _:

                    _actorGen.ResetAll();
                    _dungeonGen.GetComponent<LightGenerator>().Initialized = false;
                    _datagramHandler.SendDatagram(
                        new Hello { Player = new Player { Name = _playerName } }.CreateString(),
                        true
                    );
                    break;

                case AttkTowards towards:
                    _actorGen.AttackTowards(towards.Model.Id, towards.Model.Value.ToVector2Int());
                    break;
            }
        }

        private void OnApplicationQuit()
        {
            _pingThread.Abort();
        }
    }
}