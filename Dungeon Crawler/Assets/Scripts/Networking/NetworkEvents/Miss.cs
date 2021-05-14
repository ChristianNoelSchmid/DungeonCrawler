using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class Miss : NetworkEvent 
    {
        public DataModel<int> Model { get; set; }
        public Miss() => Model = null;
        public Miss(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new DataModel<int>
            {
                Id = int.Parse(args[0]),     
                Value = int.Parse(args[1])
            };
        }
        public string CreateString() => $"";
    }
}