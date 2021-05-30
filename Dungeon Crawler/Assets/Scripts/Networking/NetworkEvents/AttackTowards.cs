using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class AttackTowards : NetworkEvent 
    {
        public DataModel<PositionModel> Model { get; set; }
        public AttackTowards() => Model = null;
        public AttackTowards(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new DataModel<PositionModel>
            {
                Id = int.Parse(args[0]),
                Value = new PositionModel 
                { 
                    X = int.Parse(args[1]), 
                    Y = int.Parse(args[2]), 
                },
            };
        }
        public string CreateString() => $"Combat::AttackTowards::{Model.Serialize()}";
    }
}