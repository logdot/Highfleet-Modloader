#pragma once
#define DllExport __declspec( dllexport )

/**
 * The name of your mod.
 * Must be unique.
 *
 * *REQUIRED*
 */
DllExport inline const char* Name() {
    return "";
}

/**
 * The version of your mod.
 *
 * *REQUIRED*
 */
DllExport inline const char* Version() {
    return "";
}

/**
 * The version of the game your mod is intended to work on.
 *
 * *REQUIRED*
 */
DllExport inline const char* GameVersion() {
    return "";
}

/**
 * The github link to your mod.
 *
 * *OPTIONAL*
 */
DllExport inline const char* GithubLink() {
    return "";
}