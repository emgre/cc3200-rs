MEMORY
{
    SRAM (RWX) : ORIGIN = 0x20004000, LENGTH = 0x3C000
}

REGION_ALIAS("FLASH", SRAM);
REGION_ALIAS("RAM", SRAM);
