using System.Collections.Generic;
using System.Collections.ObjectModel;
using UnityEngine;

namespace DungeonCrawler.Models
{
    public class Dungeon : ISerializable
    {
        public int AttakerId { get; set; }
        public int DefenderId { get; set; }

        // The client never sends a Hit datagram, so
        // Serialize is simply implemented
        public string Serialize() => "";
    }
}