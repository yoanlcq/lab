#ifdef __cplusplus
#define EXTERN_C extern "C"
#else
#define EXTERN_C
#endif

#ifdef _WIN32
// For both constants, the type is DWORD, i.e u32. Using int avoids including windows.h.
// https://docs.nvidia.com/gameworks/content/technologies/desktop/optimus.htm
EXTERN_C __declspec(dllexport) int NvOptimusEnablement = 1;
// https://gpuopen.com/amdpowerxpressrequesthighperformance/
EXTERN_C __declspec(dllexport) int AmdPowerXpressRequestHighPerformance = 1;
#endif