using System.Collections.Generic;
using System.Collections.ObjectModel;
using UnityEngine;

namespace DungeonCrawler.Models
{
    public class HitModel : ISerializable
    {
        public int DefenderId { get; set; }
        public int HealthLeft { get; set; }

        // The client never sends a Hit datagram, so
        // Serialize is simply implemented
        public string Serialize() => "";
    }
}