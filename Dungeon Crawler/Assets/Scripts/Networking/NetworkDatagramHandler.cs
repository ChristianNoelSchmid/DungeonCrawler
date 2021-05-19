using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using DungeonCrawler.Networking.Datagrams;
using UnityEngine;

namespace DungeonCrawler.Networking
{
    /// 
    /// <summary>
    /// Handles the close-to-metal transmission of UDP
    /// datagram packets. Handles relevant flags (such as REL, ACK
    /// and RES), and automatically handles retransmission of reliable
    /// packets, and requesting retransmissions from other UDP clients.
    /// </summary>
    /// 
    /// The handler can send messages to any arbitrary server, but it can
    /// also react to messages received by connections. This is done via
    /// an event being triggered with two callbacks - one to send a new
    /// datagram to the caller, and another to send information to all
    /// other connected clients.
    /// 
    public class NetworkDatagramHandler : MonoBehaviour
    {
        [SerializeField]
        private int _port;

        [SerializeField]
        private NetworkEventHandler _eventHandler;

        [SerializeField]
        private UIDisconnectPanel _disconnectPanel;
        private bool _disconnected = false; // Set to true to allow disconnectObject to show on main thread
 
        /// <summary>
        /// Length of ticks for a packet timeout
        /// </summary>
        /// <returns>Length of a network timeout - 1 second</returns>
        private readonly double timeout = Math.Pow(10.0, 7.0);
        private Thread _listeningThread;
        private Thread _resolverThread;

        /// <summary>
        /// The client for the UDP network. Handles sending and receiving
        /// of datagrams.
        /// </summary>
        private UdpClient _client;

        /// <summary>
        /// List which contains all information about needed acknowledgements.
        /// Ensures specific messages are sent. When an ensured message is sent,
        /// it's ID (ulong) is stored in this List, as well as the time at
        /// which it was sent. Periodically, this List is checked and, upon
        /// a time which a values been in it longer than a specified timeout,
        /// it resends all resolver messages. Acknowledgements that do come remove the
        /// value from the List.
        /// </summary>
        private List<AckResolver> _resolverBuffer;
        private readonly object _listLock = new object();

        /// <summary>
        /// An index representing the total number of ensured datagrams 
        /// sent. This is updated on both the client side and server side,
        /// to ensure that datagrams which need acknowledgements are not lost 
        /// in the network.
        /// </summary>
        private ulong _ackExpectedIndex;

        /// <summary>
        /// Represents the indices of the local client's current index count
        /// of ack datagrams, per connection. A dictionary is needed as the local
        /// client may send different amounts of ensured datagrams to each client.
        /// </summary>
        private ulong _ackCurrentIndex;

        public EventHandler<DatagramCallback> MessageRecieved;
        public bool IsListening { get; set; } = false;

        /// <summary>
        /// Creates the NetworkDatagramHandler. This can be either a client
        /// (sending an recieving data from only one source - the server), or
        /// a server (sending and recieving data from and to multiple clients)
        /// </summary>
        /// <param name="port">The port which is being connected to</param>
        /// <param name="extHostname">The server's hostname
        /// Setting this value will result in a client handler</param>
        private void Awake()
        {
            _resolverBuffer = new List<AckResolver>();
            _ackExpectedIndex = 0;
            _ackCurrentIndex = 0;

            // Start the AckResolver, and Listening Thread
            _resolverThread = new Thread(StartAckResolver){ IsBackground = true };
        }

        private void Update()
        {
            if (_disconnected && !_disconnectPanel.IsVisible)
                _disconnectPanel.SetVisible(true);
        }

        public bool AttemptSignin(string name, string ipAddr) => StartHandler(name, ipAddr);

        private bool StartHandler(string name, string ip)
        {
            try
            {
                _client = new UdpClient(ip, _port);

                _listeningThread = new Thread(StartReceiving) { IsBackground = true };
                _listeningThread.Start();
            }
            catch (Exception ex)
            {
                Debug.LogError(ex.Message);
                _listeningThread?.Abort();
                return false;        
            }

            IsListening = true;
            _resolverThread.Start();
            _eventHandler.StartHandler(name);

            return true;
        }

        private void OnApplicationQuit()
        {
            _resolverThread?.Abort();
            _listeningThread?.Abort();
        }

        /// <summary>
        /// Generically sends a datagram to the specified end points, either reliable or unreliable.
        /// </summary>
        /// <param name="message">The message to send</param>
        /// <param name="isReliable">Whether the handler should ensure 
        /// the message reaches its intended recipient</param>
        /// <param name="endPoints">The client(s) to send the datagram to</param>
        public void SendDatagram(string message, bool isReliable)
        {
            byte[] msgBytes;

            if (isReliable)
            {
                if (_resolverBuffer.Count >= 100) return;

                // Append the ack index count to the message, to communicate to the
                // client what reliable datagram count it represents
                msgBytes = Encoding.ASCII.GetBytes(Reliable.CreateString(_ackCurrentIndex, message));

                // Add a new AckResolver to the resolver buffer, which will
                // resend the datagram if the timeout is reached
                // before receiving an ACK
                AddAckResolver(
                    new AckResolver()
                    {
                        AckIndex = _ackCurrentIndex,
                        TicksStart = DateTime.Now.Ticks,
                        Message = message
                    }
                );

                _ackCurrentIndex += 1;
            }
            else
                msgBytes = Encoding.ASCII.GetBytes(Unreliable.CreateString(message));

            try
            {
                _client.Send(msgBytes, msgBytes.Length);
            }
            catch (SocketException)
            {
                //_displayDisconnection.Display();
            }
        }

        public void SendAck(ulong index)
        {
            byte[] msgBytes;
            msgBytes = Encoding.ASCII.GetBytes(Ack.CreateString(index));

            try
            {
                _client.Send(msgBytes, msgBytes.Length);
            }
            catch (SocketException)
            {
                //_displayDisconnection.Display();
            }
        }

        /// <summary>
        /// Sends a datagram with the specified AckResolver info
        /// The datagram must be sent reliably, but the AckResolver
        /// is already in the buffer, so there's no need to add it again.
        /// </summary>
        /// <param name="resolver">The AckResolver with the datagram info</param>
        private void SendDatagram(AckResolver resolver)
        {
            byte[] msgBytes;

            // Append the ack index count to the message, to communicate to the
            // client what reliable datagram count it represents
            msgBytes = Encoding.ASCII.GetBytes(Reliable.CreateString(resolver.AckIndex, resolver.Message));
            _client.Send(msgBytes, msgBytes.Length);
        }

        /// <summary>
        /// Adds a new AckResolver to the resolver buffer
        /// </summary>
        /// <param name="resolver">The new AckResolver to add</param>
        private void AddAckResolver(AckResolver resolver)
        {
            // Retrieve the AckResolver List (if it exists) for
            // the specified connection, and add the new AckResolver
            // to the List
            lock(_listLock) { 
                _resolverBuffer.Add(resolver); 
                _resolverBuffer.Sort((ack1, ack2) => ack1.AckIndex.CompareTo(ack2.AckIndex));
            }
        }

        private void AcceptAck(ulong ack)
        {
            int index;
            lock(_listLock)
            {
                index = _resolverBuffer.FindIndex(res => res.AckIndex == ack);
                if(index != -1) _resolverBuffer.RemoveAt(index);
            }
        }

        /// <summary>
        /// Resends the earliest reliable datagram, using
        /// the ack resolver.
        /// </summary>
        /// <param name="endPoint">The end point to send the datagram to</param>
        private void ResendRel()
        {
            AckResolver resolver;
            if((resolver = _resolverBuffer.First()) != null)
                SendDatagram(resolver);
        }

        /// <summary>
        /// Converts an incoming datagram into the correct
        /// datagram type, based on which tag is attached
        /// to the message.
        /// </summary>
        /// <param name="datagram">The sequence of bytes representing the datagram message</param>
        /// <returns></returns>
        private Datagram ParseDatagram(byte[] datagram)
        {
            string msg = Encoding.ASCII.GetString(datagram).Trim();
            try
            {
                switch(msg)
                {
                    case string s when s.StartsWith("DRP"):
                        return new Drop();
                    case string s when s.StartsWith("ACK"):
                        return new Ack(s.Substring(5));
                    case string s when s.StartsWith("REL"):
                        return new Reliable(s.Substring(5));
                    case string s when s.StartsWith("RES"):
                        return new Resend();
                    case string s when s.StartsWith("UNR"):
                        return new Unreliable(s.Substring(5));
                    default:
                        return null;
                };
            }
            catch(Exception) { return null; }
        }

        #region Multithreaded Sub-processes
        /// <summary>
        /// The system which handles recieving datagrams, converting
        /// them to their appropriate type, and passing basic information
        /// back to the client and / or the local application.
        /// </summary>
        private void StartReceiving()
        {
            IPEndPoint endPoint = null;
            byte[] bytes;
            Datagram datagram;

            while(true)
            {
                if(!IsListening) continue;

                try
                {
                    bytes = _client.Receive(ref endPoint); 
                
                    if(!IsListening) continue;

                    datagram = ParseDatagram(bytes);

                    switch(datagram)
                    {
                        case Drop _:
                            _disconnected = true;                                             break;
                    
                        // Datagram is reliable, but the ACK index is too high.
                        // Ask client to resend awaiting data.
                        case Reliable rel when rel.AckIndex > _ackExpectedIndex:
                            SendDatagram(Resend.CreateString(), false);                        break;

                        // Message is reliable, but the ACK index is too low.
                        // Already recieved datagram: simply resend ACK and return
                        case Reliable rel when rel.AckIndex < _ackExpectedIndex:
                            SendDatagram(Ack.CreateString(rel.AckIndex), false);               break;

                        // Message is reliable, and was the expected index.
                        // Accept message, send ACK, and invoke MessageRecieved event
                        case Reliable rel:
                            _ackExpectedIndex += 1;
                            SendAck(rel.AckIndex);
                            MessageRecieved.Invoke(null, new DatagramCallback
                            {
                                Data = rel.Data,
                                SendToServer = (data, isRel) => SendDatagram(data, isRel)
                            });                                                               break;

                        // Message contains request to resend packages
                        // Resend earliest package
                        case Resend _: ResendRel();                                           break;

                        // Message is an acknowledgement datagram
                        // Accept and remove awaiting AckResolver
                        case Ack ack: AcceptAck(ack.AckIndex);                                break;

                        // Message is unreliable - invoke MessageRecieved event
                        case Unreliable unrel: 
                            MessageRecieved.Invoke(null, new DatagramCallback
                            {
                                Data = unrel.Data,
                                SendToServer = (data, isRel) => SendDatagram(data, isRel)
                            });                                                               break; 
                    }
                }
                catch(SocketException se)
                {
                    //_displayDisconnection.Display();
                    throw se;
                }
            }
        }

        /// <summary>
        /// Initiates the AckListener, which will routinely send
        /// reliable messages every time its AckResolver times out
        /// in the resolver buffer.
        /// </summary>
        private void StartAckResolver()
        {
            AckResolver firstResolver;

            while(true)
            {
                Thread.Sleep(100);

                firstResolver = _resolverBuffer.FirstOrDefault();
                if(DateTime.Now.Ticks - firstResolver?.TicksStart > timeout)
                {
                    foreach(var resolver in _resolverBuffer)
                        SendDatagram(resolver);
                }
            }
        }
        #endregion
    }
}