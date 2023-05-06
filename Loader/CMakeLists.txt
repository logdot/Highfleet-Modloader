set(LoaderName "Modloader")
add_library(${LoaderName} SHARED)

target_sources(${LoaderName}
        PUBLIC
            DllMain.cpp
)
