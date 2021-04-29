using System;
using System.Collections.Generic;
using System.Globalization;

using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Networking.NetworkEvents
{
    /// <summary>
    /// NetworkEvent representing the Server's welcome to a client,
    /// providing all relevant information to the ServerState of the
    /// game, and assigning the Client an Id.
    /// </summary>
    public class Welcome : NetworkEvent 
    {
        public DataModel<Dungeon> Model { get; set; }
        public Welcome() => Model = null;
        public Welcome(string value) 
        {
            var segs = value.Split(new string[] { "::" }, StringSplitOptions.None);
            var id = int.Parse(segs[0]);

            var path_count = int.Parse(segs[1]);
            var paths = new List<Vector2Int>();
            Vector2Int entrance;
            Vector2Int exit;
            int i = 2;

            for(; i < path_count * 2 + 1; i += 2) 
            {
                paths.Add(
                    new Vector2Int (
                        int.Parse(segs[i]),
                        int.Parse(segs[i + 1])
                    )
                );
            }

            entrance = new Vector2Int(int.Parse(segs[i]), int.Parse(segs[i+1]));
            exit = new Vector2Int(int.Parse(segs[i+2]), int.Parse(segs[i+3]));

            Model = new DataModel<Dungeon>
            {
                Id = id,
                Value = new Dungeon
                {
                    Paths = paths,
                    Entrance = entrance,
                    Exit = exit,
                },
            };
        }

        public string CreateString() => $"Welcome::{Model.Serialize()}";
    }
}