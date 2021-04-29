using System;

namespace DungeonCrawler.Networking
{
    /// <summary>
    /// The callback used when the server has sent information
    /// to a client. Contains the unparsed data from the datagram
    /// and a function which allows easy response to the server
    /// (if applicable).
    /// </summary>
    public class DatagramCallback
    {
        public string Data { get; set; }
        public Action<string, bool> SendToServer { get; set; }
    }
}