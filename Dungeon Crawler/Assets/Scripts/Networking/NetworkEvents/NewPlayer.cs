using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class NewPlayer : NetworkEvent 
    {
        public Player Model { get; set; }
        public NewPlayer() => Model = null;
        public NewPlayer(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new Player
            {
                Id = int.Parse(args[0]),
                Position = new PositionModel
                {
                    X = int.Parse(args[1]), 
                    Y = int.Parse(args[2]),
                    Direction = ((Direction)int.Parse(args[3])),
                }
            };
        }
        public string CreateString() => $"";
    }
}