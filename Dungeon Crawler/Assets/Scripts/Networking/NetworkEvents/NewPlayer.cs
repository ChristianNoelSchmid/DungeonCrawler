using System;
using UnityEngine;

using DungeonCrawler.Models;
using DungeonCrawler.Monobehaviours;
using Assets.Scripts.Models;

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
                Name = args[1].Trim(),
                Status = (Status)int.Parse(args[2]),
            };
        }
        public string CreateString() => $"";
    }
}