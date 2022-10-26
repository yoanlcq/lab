BUILD_PRX := 1
TARGET := experiment
OBJS := experiment.o

INCDIR :=
CFLAGS := -Wall -Wdouble-promotion -O3 -g
CXXFLAGS := $(CFLAGS) -fno-exceptions -fno-rtti
ASFLAGS := $(CFLAGS)

LIBDIR :=
LDFLAGS :=
LIBS := -lpspgu -lpspfpu -lpspgum_vfpu -lpspvfpu

EXTRA_TARGETS := EBOOT.PBP
PSP_EBOOT_TITLE := Experiment

PSPSDK := $(shell psp-config --pspsdk-path)
include $(PSPSDK)/lib/build.mak
