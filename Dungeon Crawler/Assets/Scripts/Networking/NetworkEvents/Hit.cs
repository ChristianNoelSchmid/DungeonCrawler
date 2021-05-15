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
        public DataModel<HitModel> Model { get; set; }
        public Hit() => Model = null;
        public Hit(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new DataModel<HitModel>
            {
                Id = int.Parse(args[0]),     
                Value = new HitModel
                {
                    DefenderId = int.Parse(args[1]),
                    HealthLeft = int.Parse(args[2]),
                },
            };
        }
        public string CreateString() => $"";
    }
}