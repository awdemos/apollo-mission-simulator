#include <stdint.h>
#include <stdlib.h>
#include <string.h>

struct agc_t;
int agc_engine_init(struct agc_t *State, const char *RomImage, const char *CoreDump, int AllOrErasable);
int agc_engine(struct agc_t *State);
int agc_load_binfile(struct agc_t *State, const char *RomImage);
int ReadIO(struct agc_t *State, int Address);
void WriteIO(struct agc_t *State, int Address, int Value);
void CpuWriteIO(struct agc_t *State, int Address, int Value);
void MakeCoreDump(struct agc_t *State, const char *CoreDump);
int16_t OverflowCorrected(int Value);
int SignExtend(int16_t Word);
int AddSP16(int Addend1, int Addend2);
void UnprogrammedIncrement(struct agc_t *State, int Counter, int IncType);
void ChannelOutput(struct agc_t *State, int Channel, int Value);
int ChannelInput(struct agc_t *State);
void ChannelRoutine(struct agc_t *State);

void* agc_state_alloc(void) {
    return calloc(1, 200000);
}

void agc_state_free(void* state) {
    free(state);
}

void ChannelOutput(struct agc_t *State, int Channel, int Value) {
    (void)State;
    (void)Channel;
    (void)Value;
}

int ChannelInput(struct agc_t *State) {
    (void)State;
    return 0;
}

void ChannelRoutine(struct agc_t *State) {
    (void)State;
}

int BacktraceAdd(int FrameCount, int Address, int Flag) {
    (void)FrameCount;
    (void)Address;
    (void)Flag;
    return 0;
}

int DbgLinearFixedAddr(int Bank, int Address) {
    (void)Bank;
    (void)Address;
    return 0;
}

const char* DbgGetFrameNameByAddr(int Address) {
    (void)Address;
    return "";
}

const char* SourcePathName = "";

const char* NormalizeSourceName(const char *Name) {
    return Name;
}

int ResolveLineAGC(int Address, char *Line, int *Page, int *LineNumber) {
    (void)Address;
    (void)Line;
    (void)Page;
    (void)LineNumber;
    return 0;
}

void ShiftToDeda(struct agc_t *State, int Channel, int Value) {
    (void)State;
    (void)Channel;
    (void)Value;
}

void RequestRadarData(struct agc_t *State) {
    (void)State;
}
