using System.Collections.Generic;
using System.Linq;

using UnityEngine;

using DungeonCrawler.Models;

namespace DungeonCrawler.Monobehaviours
{
    public class DungeonGenerator : MonoBehaviour
    {
        [SerializeField]
        private Voxel _wallTemplate;

        [SerializeField]
        private Voxel _floorTemplate;

        [SerializeField]
        private GameObject _trapdoor;

        [SerializeField]
        private LightGenerator _lightGenerator;
        private List<Voxel> _voxels;

        private Dungeon _dungeon;
        public Dungeon Dungeon 
        {
            get => _dungeon;
            set
            {
                _dungeon = value;
                Generate();
            }
        }
        private Transform _transform;

        private void Awake() 
        {
            _transform = transform;
        }

        private void Generate()
        {
            if(_voxels != null && _voxels.Count > 0) 
            {
                foreach(var voxel in _voxels)
                {
                    Destroy(voxel.gameObject);
                    Obstacles.RemoveObstacle(voxel.transform);
                }
            }

            _voxels = new List<Voxel>();

            var width = _dungeon.Paths.Max(p => p.x);
            var height = _dungeon.Paths.Max(p => p.y);

            for(int x = -20; x <= width + 20; ++x) 
            {
                for(int y = -20; y <= height + 20; ++y) 
                {
                    if(!_dungeon.Paths.Contains(new Vector2Int(x, y)))
                    {
                        var wallObject = Instantiate(_wallTemplate, new Vector3(x, y), Quaternion.identity, _transform);
                        _voxels.Add(wallObject.GetComponent<Voxel>());

                        Obstacles.UpdateObstacle(wallObject.transform, new Vector2Int(x, y));
                    }
                    else 
                        _voxels.Add(Instantiate(_floorTemplate, new Vector3(x, y), Quaternion.identity, _transform));
                }
            }

            _trapdoor.GetComponent<GridPosition>().Value = _dungeon.Exit;
            _lightGenerator.ImportDungeon(_voxels);
        }
    }
}