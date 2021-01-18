#pragma once

#ifdef _MSC_VER
#define FORWARDED_EXPORT_WITH_ORDINAL(exp_name, target_name, ordinal) __pragma(comment(linker, "/export:" #exp_name "=" #target_name ",@" #ordinal))
#define FORWARDED_EXPORT(exp_name, target_name) __pragma(comment(linker, "/export:" #exp_name "=" #target_name))
#endif
#ifdef __GNUC__
#define START_EXPORTS() asm(".section .drectve"
#define END_EXPORTS() );
//#define FORWARDED_EXPORT_WITH_ORDINAL(exp_name, target_name, ordinal) asm(".section .drectve\n\t.ascii \" -export:" #exp_name "=" #target_name "@" #ordinal "\"");
#define FORWARDED_EXPORT_WITH_ORDINAL(exp_name, target_name, ordinal) asm(".section .drectve\n\t.ascii \" -export:" #exp_name "=" #target_name "\"");

#define FORWARDED_EXPORT(exp_name, target_name) asm(".section .drectve\n\t.ascii \" -export:" #exp_name "=" #target_name "\"");
//#define FORWARDED_EXPORT_WITH_ORDINAL(exp_name, target_name, ordinal)
#endif
