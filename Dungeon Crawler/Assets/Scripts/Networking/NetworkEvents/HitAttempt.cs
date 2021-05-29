using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class HitAttempt : NetworkEvent 
    {
        public DataModel<MissModel> Model { get; set; }
        public HitAttempt() => Model = null;
        public HitAttempt(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new DataModel<MissModel>
            {
                Id = int.Parse(args[0]),     
                Value = new MissModel
                {
                    DefenderId = int.Parse(args[1]),
                },
            };
        }
        public string CreateString() => $"AttemptHit::{Model.Serialize()}";
    }
}