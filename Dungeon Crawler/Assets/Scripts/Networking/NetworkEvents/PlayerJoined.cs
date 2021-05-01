using System;
using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents
{
    /// <summary>
    /// NetworkEvent representing a new Client that
    /// has joined the game.
    /// </summary>
    public class PlayerJoined : NetworkEvent 
    { 
        public PositionModel Position { get; set; }

        public PlayerJoined() => Position = null;
        public PlayerJoined(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Position = new PositionModel
            {
                X = int.Parse(args[0]), 
                Y = int.Parse(args[1])
            };
        }

        public string CreateString() => $"PlayerJoined::{Position.Serialize()}";
    }
}