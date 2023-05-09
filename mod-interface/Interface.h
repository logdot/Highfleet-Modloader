/**
 * This is the interface between your mod and the modloader.
 *
 * The modloader will call the interface top to bottom.
 * If any of your functions returns false, the modloader will immediately eject your mod and free it from memory.
 * UnInject will *always* be run before the modloader ejects your mod.
 * All the functions will be run from within the game process.
 *
 * The order in which mods are loaded is not guaranteed to be consistent.
 */

#pragma once
#define DllExport __declspec( dllexport )

#include <iostream>

//! The logging stream assigned by the modloader.
extern std::ostream* log_stream;

/**
 * Called by the modloader to set the mod's logging stream.
 *
 * @param ostream The stream to use for logging
 */
DllExport inline void __cdecl SetLogger(std::ostream* ostream) {
    log_stream = ostream;
}

/**
 * Should define any code that need to be done to setup the mod.
 * For example, loading and parsing any configuration files.
 *
 * @return Whether the function completed successfully
 */
DllExport bool Setup();

/**
 * Should define any code that needs to be done right before injection.
 *
 * Guaranteed to be run directly before Inject
 *
 * @return Whether the function completed successfully
 */
DllExport bool PreInject();

/**
 * Should define the code that does the actual injection.
 *
 * @return Whether the function completed successfully
 */
DllExport bool Inject();

/**
 * Should define any code that need to be done right after injection.
 *
 * Guaranteed to be run directly after Inject.
 *
 * @return Whether the function completed successfully
 */
DllExport bool PostInject();

/**
 * Should define the code that undoes the injection.
 *
 * This function needs to uphold these two properties:
 *   - Inject() => UnInject() Should result in no change to the game's code
 *   - UnInject()             Should result in no change to the game's code
 *
 * That is to say that UnInject should *only* make changes to code affected by Inject
 *
 * @return Whether the function completed successfully
 */
DllExport bool UnInject();
