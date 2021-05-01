using System.Collections;
using System.Collections.Generic;
using System.Linq;

using UnityEngine;

using DungeonCrawler.Monobehaviours;

namespace DungeonCrawler.Monobehaviours
{
    public class LightGenerator : MonoBehaviour
    {
        private Dictionary<Vector2Int, Dictionary<Light, float>> _lightValues;
        private Dictionary<Vector2Int, Voxel> _voxels;
        private Dictionary<Vector2Int, float> _lightUpdates;

        private readonly WaitForSeconds _waitForInterval = new WaitForSeconds(0.1f);
        private bool _initialized = false;

        public void ImportDungeon(IEnumerable<Voxel> voxels)
        {
            _lightValues = new Dictionary<Vector2Int, Dictionary<Light, float>>();
            _voxels = new Dictionary<Vector2Int, Voxel>();
            _lightUpdates = new Dictionary<Vector2Int, float>();

            foreach(var voxel in voxels)
            {
                var pos = new Vector2Int(
                    Mathf.RoundToInt(voxel.transform.position.x), 
                    Mathf.RoundToInt((int)voxel.transform.position.y));

                _voxels.Add(pos, voxel);
                voxel.Renderer.color = Color.black;
                _lightValues.Add(pos, new Dictionary<Light, float>());
            }

            _initialized = true;
        }

        public void UpdateLight(DungeonCrawler.Monobehaviours.Light light)
        {
            if(!_initialized) return;

            var position = light.Position.Value;
            var intensityDecrement = (1.0f / light.Range * light.Intensity);
            var updatedSquares = new HashSet<Vector2Int>();

            for(float f = 0; f < 2.0f * Mathf.PI; f += (2.0f * Mathf.PI / ((float)light.Range * 8.0f)))
            {
                float x = Mathf.Cos(f);
                float y = Mathf.Sin(f);

                float sqIntensity = light.Intensity;
                bool hitWall = false;
                for(int i = 1; i <= light.Range + 1; ++i)
                {
                    Vector2Int sqPosition = position + new Vector2Int(
                        (int)Mathf.Ceil(x * i),
                        (int)Mathf.Ceil(y * i)
                    );

                    sqIntensity -= intensityDecrement;
                    if(_voxels.ContainsKey(sqPosition))
                    {
                        if(_voxels[sqPosition].IsWall || hitWall)
                        {
                            sqIntensity /= 3.0f; 
                            hitWall = true;
                        }
                    }
                    else
                        continue;
                    
                    if (updatedSquares.Contains(sqPosition))
                        continue;
                    
                    if(i > light.Range)
                        _lightValues[sqPosition][light] = 0.0f;
                    else 
                    {
                        _lightValues[sqPosition][light] = sqIntensity;
                        updatedSquares.Add(sqPosition);
                    }

                    float max = 0.0f;
                    if(_lightValues.ContainsKey(sqPosition) && _lightValues[sqPosition].Count > 0)
                        max = _lightValues[sqPosition].Values.Max();

                    _lightUpdates[sqPosition] = max;
                }
            }
        }

        private void Update()
        {
            if(!_initialized) return;

            var keys = _lightUpdates.Keys.ToArray();
            foreach(var pos in keys)
            {
                if(_voxels[pos].LightUpdating) continue;
                StartCoroutine(UpdateSquare(pos, _voxels[pos], _lightUpdates[pos]));
                _lightUpdates.Remove(pos);
            }
        }

        private IEnumerator UpdateSquare(Vector2Int pos, Voxel voxel, float newValue)
        {
            voxel.LightUpdating = true;

            var oldColor = voxel.Renderer.color;
            var newColor = new Color(newValue, newValue, newValue);
            for(float i = 0.0f; i < 1.0f; i += 0.2f)
            {
                voxel.Renderer.color = Color.Lerp(oldColor, newColor, i);
                yield return _waitForInterval;
                if(_lightUpdates.ContainsKey(pos))
                {
                    oldColor = voxel.Renderer.color;
                    float newInt = _lightUpdates[pos];
                    newColor = new Color(newInt, newInt, newInt);

                    i = 0.0f;

                    _lightUpdates.Remove(pos);
                }
            }
            voxel.Renderer.color = newColor;

            voxel.LightUpdating = false;
        }

        public float SquareBrightness(Vector2Int pos)
        {
            if(_voxels != null && _voxels.ContainsKey(pos))
                return _voxels[pos].Renderer.color.r;
            return 0.0f;
        }
    }
}