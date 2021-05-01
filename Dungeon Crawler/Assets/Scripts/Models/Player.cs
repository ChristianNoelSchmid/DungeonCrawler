using System.Collections.Generic;
using System.Collections.ObjectModel;
using UnityEngine;

namespace DungeonCrawler.Models
{
    public class Player : ISerializable
    {
        public int Id { get; set; }
        public PositionModel Position { get; set; }

        // The client never sends a Player datagram, so
        // Serialize is simply implemented
        public string Serialize() => "";
    }
}