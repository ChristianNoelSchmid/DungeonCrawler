using System;
using System.Collections;
using System.Collections.Generic;

using UnityEngine;

using DungeonCrawler.Models;
using DungeonCrawler.Networking.NetworkEvents;
using DungeonCrawler.Monobehaviours;

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

        [SerializeField]
        private bool _networkingEnabled = true;

        [SerializeField]
        private DungeonGenerator _dungeonGen;

        [SerializeField]
        private ActorGenerator _actorGen;

        [SerializeField]
        private GridPosition _playerPosition;
        private Vector2Int _prevPlayerPosition;

        private const float _playerUpdateIntevalSeconds = 0.1f;

        private int _playerId;
        private Queue<DatagramCallback> _callbackQueue;

        private void Awake()
        {
            if(!_networkingEnabled) return;

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
        public void StartHandler()
        {
            if(!_networkingEnabled) return;             
            _datagramHandler.SendDatagram(
                new Hello().CreateString(),
                true
            );
        }

        private void Update()
        {
            if(!_networkingEnabled) return;
            var mousePos = Camera.main.ScreenToWorldPoint(Input.mousePosition);
            var mousePosInt = new Vector2Int(Mathf.RoundToInt(mousePos.x), Mathf.RoundToInt(mousePos.y));

            if(Input.GetMouseButtonDown(0)) {
                _datagramHandler.SendDatagram(new RequestMove { Model = mousePosInt }.CreateString(), false);
            }

            // If there is an event in the queue, call it
            while(_callbackQueue.Count > 0)
                TransferEvent(_callbackQueue.Dequeue());
        }

        // Periodically sends ping information to the Server, 
        // with the local Player's GridPosition
        private IEnumerator BeginPinging()
        {
            while(true)
            {
                if(_prevPlayerPosition != _playerPosition.Value)
                {
                    _datagramHandler.SendDatagram(
                        new Moved
                        {
                            Model = new DataModel<PositionModel>
                            {
                                Id = _playerId,
                                Value = _playerPosition.ToPositionModel()
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
            Debug.Log(text);
            var command = text.Split(new string [] { "::" }, 2, StringSplitOptions.None)[0];
            var args = text.Substring(command.Length + 2);
            try 
            {
                return command switch
                {
                    "Welcome"    =>     new Welcome(args),
                    "NewPlayer"  =>     new NewPlayer(args),
                    "NewMonster" =>     new NewMonster(args),
                    "PlayerLeft" =>     new PlayerLeft(args),
                    "Moved"      =>     new Moved(args),
                    _            =>     null
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
                    _playerPosition.Value = _dungeonGen.Dungeon.Entrance;
                    _playerId = welcome.Model.Id;

                    StartCoroutine(BeginPinging());
                    break;
                
                case NewPlayer newPlayer:

                    _actorGen.SpawnPlayer(newPlayer.Model);
                    break;

                case NewMonster newMonster:

                    _actorGen.SpawnMonster(newMonster.Model);
                    break;

                case Moved moved:
                    if(moved.Model.Id == _playerId)
                        _playerPosition.FromPositionModel(moved.Model.Value);
                    else
                        _actorGen.UpdatePosition(moved.Model.Id, moved.Model.Value);
                    break;

                case PlayerLeft left: // On PlayerLeft, remove the Client's marker from the PlayerConnections

                    break;
            }
        }
    }
}