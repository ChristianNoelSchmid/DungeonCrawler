using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class NewMonster : NetworkEvent 
    {
        public MonsterInstance Model { get; set; }
        public NewMonster() => Model = null;
        public NewMonster(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new MonsterInstance
            {
                TemplateId = int.Parse(args[0]),
                InstanceId = int.Parse(args[1]),
            };
        }
        public string CreateString() => $"";
    }
}