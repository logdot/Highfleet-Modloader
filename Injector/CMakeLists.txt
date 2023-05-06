set(InjectorName "ModloaderInjector")
add_executable(${InjectorName} "")

target_sources(${InjectorName}
        PUBLIC
            Main.cpp
            Inject.cpp
)
