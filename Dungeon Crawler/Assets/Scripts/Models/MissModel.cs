using System.Collections.Generic;
using System.Collections.ObjectModel;
using UnityEngine;

namespace DungeonCrawler.Models
{
    public class MissModel : ISerializable
    {
        public int DefenderId { get; set; }

        // The client never sends a Miss datagram, so
        // Serialize is simply implemented
        public string Serialize() => "";
    }
}