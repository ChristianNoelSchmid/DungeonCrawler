using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class Hit : NetworkEvent 
    {
        public DataModel<Tuple<int, int>> Model { get; set; }
        public Hit() => Model = null;
        public Hit(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new DataModel<Tuple<int, int>>
            {
                Id = int.Parse(args[0]),     
                Value = Tuple.Create(int.Parse(args[1]), int.Parse(args[2]))
            };
        }
        public string CreateString() => $"";
    }
}