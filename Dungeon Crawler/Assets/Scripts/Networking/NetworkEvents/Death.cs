using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has died
    /// </summary>
    public class Dead : NetworkEvent 
    {
        public DataModel Model { get; set; }
        public Dead() => Model = null;
        public Dead(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new DataModel
            {
                Id = int.Parse(args[0]),     
            };
        }
        public string CreateString() => $"";
    }
}