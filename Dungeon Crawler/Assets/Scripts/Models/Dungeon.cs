using System.Collections.Generic;
using System.Collections.ObjectModel;
using UnityEngine;

namespace DungeonCrawler.Models
{
    public class Dungeon : ISerializable
    {
        public List<Vector2Int> Paths { get; set; }
        public Vector2Int Entrance { get; set; }
        public Vector2Int Exit { get; set; }

        // The client never sends a Dungeon datagram, so
        // Serialize is simply implemented
        public string Serialize() => "";
    }
}