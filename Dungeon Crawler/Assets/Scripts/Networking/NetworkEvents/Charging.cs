using DungeonCrawler.Models;
using DungeonCrawler.Networking.NetworkEvents;
using System;

namespace Assets.Scripts.Networking.NetworkEvents
{
    public class Charging : NetworkEvent
    {
        public DataModel Model;
        public Charging() => Model = null;
        public Charging(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new DataModel
            {
                Id = int.Parse(args[0]),
            };
        }
        public string CreateString() => $"Combat::Charging::{Model.Id}";
    }
}
