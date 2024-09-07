## GLTR

gltr is a glTF parsing library. it is a WIP and is only going t handle working with the file format and contents 
of the JSON version of the file (.gltf), not the binary (.glb) and will not handle any sort of rendering of the contents
or converting them to other formats

the inspiration for this crate was that I had a large number of objects in a single gltf scene that
i wanted to be decomposed into their own individual gltf files so they could be managed independently.
This was designed to help with that - find all nodes in a gltf scene, find their dependant buffers, bufferviews, materials
etc and create new gltf files for them and write them to disk
