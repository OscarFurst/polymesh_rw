// Gmsh project created on Sat Jun  8 16:57:12 2024
//+
SetFactory("OpenCASCADE");
Box(1) = {0, 0, 0, 10, 1, 1};
//+
Extrude {0, -5, 0} {
  Surface{3}; 
}
//+
Physical Volume("channel", 21) = {1};
//+
Physical Volume("metal", 22) = {2};
//+
Physical Surface("sides", 23) = {9, 7, 6, 5};
//+
Physical Surface("inlet", 24) = {1};
//+
Physical Surface("outlet", 25) = {2};
//+
Physical Surface("top", 26) = {4};
//+
Physical Surface("bottom", 27) = {11};
//+
Physical Surface("metal_in", 28) = {10};
//+
Physical Surface("metal_out", 29) = {8};
//+
Physical Surface("interface", 30) = {3};
