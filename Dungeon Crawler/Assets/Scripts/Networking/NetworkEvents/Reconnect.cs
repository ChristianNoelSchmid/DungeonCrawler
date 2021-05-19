using System;
using System.Collections.Generic;
using System.Globalization;

using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents
{
    /// <summary>
    /// NetworkEvent representing the Server's welcome to a client,
    /// providing all relevant information to the ServerState of the
    /// game, and assigning the Client an Id.
    /// </summary>
    public class Reconnect : NetworkEvent
    {
        public string CreateString() => "";
    }
}