using System;
using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents 
{
    /// <summary>
    /// NetworkEvent representing a Client that
    /// has pinged the Server
    /// </summary>
    public class RequestMove : NetworkEvent 
    {
        public Vector2Int Model { get; set; }
        public RequestMove() => Model = Vector2Int.zero;
        public RequestMove(string value)
        {
            string [] args = value.Split(new string[] { "::" }, StringSplitOptions.None);
            Model = new Vector2Int(int.Parse(args[0]), int.Parse(args[1]));
        }
        public string CreateString() => $"RequestMove::{Model.x}::{Model.y}";
    }
}