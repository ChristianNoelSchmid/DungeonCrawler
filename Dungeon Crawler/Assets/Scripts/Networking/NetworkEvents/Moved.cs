using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class Moved : NetworkEvent 
    {
        public DataModel<Position> Model { get; set; }
        public Moved() => Model = null;
        public Moved(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new DataModel<Position>
            {
                Id = int.Parse(args[0]),
                Value = new Position { X = int.Parse(args[1]), Y = int.Parse(args[2]) },
            };
        }
        public string CreateString() => $"Moved::{Model.Serialize()}";
    }
}