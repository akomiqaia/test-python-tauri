<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { open } from "@tauri-apps/plugin-dialog";
    import { readTextFile } from "@tauri-apps/plugin-fs";

    let fileContent = "";
    let fileName = "Untitled";
    let isPythonInstalled = false;
    let isInstallingPython = false;
    let setupComplete = false;
    let installationError = "";
    let installationProgress = { stage: "", progress: 0 };
    async function installPythonFromRust() {
        // Implement Python installation
        await invoke("start_python_server");
    }
    async function checkPythonVersion() {
        // Check if Python is installed in the app's data directory
        isPythonInstalled = await invoke("check_python_installed");
    }
    async function installPython() {
        if (isInstallingPython) return; // Prevent multiple installations
        isInstallingPython = true;
        installationError = "";
        try {
            const unlisten = await listen(
                "python_install_progress",
                (event) => {
                    installationProgress = event.payload as {
                        stage: string;
                        progress: number;
                    };
                },
            );

            // Start the installation process in the background
            invoke("install_python_background")
                .then(() => {
                    isPythonInstalled = true;
                    isInstallingPython = false;
                    unlisten();
                })
                .catch((error) => {
                    console.error("Failed to install Python:", error);
                    installationError = `Failed to install Python: ${error}`;
                    isInstallingPython = false;
                    unlisten();
                });

            // Allow the user to continue using the app
            return;
        } catch (error) {
            console.error("Failed to start Python installation:", error);
            installationError = `Failed to start Python installation: ${error}`;
            isInstallingPython = false;
        }
    }

    async function setupEnvironment() {
        // Implement environment setup
        setupComplete = true;
    }

    async function openFile() {
        // ... (keep existing openFile function)
    }

    function handleInput(event: Event) {
        // ... (keep existing handleInput function)
    }

    // Check Python version when component mounts
    checkPythonVersion();

    let requestResult = "no result";

    async function getSimpleRequest() {
        const response = await fetch("http://localhost:8000/");
        const data = await response.json();
        console.log(data);
        requestResult = data.Hello;
    }
</script>

<div class="editor">
    <div class="sidebar">
        {#if !isPythonInstalled}
            <div class="python-installation">
                <h2>Python 3.11 Required</h2>
                <p>
                    This application requires Python 3.11. Would you like to
                    install it now?
                </p>

                <button on:click={getSimpleRequest}>
                    simple request {requestResult}</button
                >
                {#if isInstallingPython}
                    <div class="progress-bar">
                        <div
                            class="progress"
                            style="width: {installationProgress.progress}%"
                        ></div>
                    </div>
                    <p>{installationProgress.stage}</p>
                {/if}
                {#if installationError}
                    <p class="error">{installationError}</p>
                {/if}
            </div>
        {:else if !setupComplete}
            <button on:click={setupEnvironment}>Setup Environment</button>
        {:else}
            <button on:click={openFile}>Open File</button>
            <div class="file-list">
                <div class="file">{fileName}</div>
            </div>
        {/if}
    </div>
    <div class="main-content">
        <div class="tabs">
            <div class="tab active">{fileName}</div>
        </div>
        <textarea
            value={fileContent}
            on:input={handleInput}
            placeholder="Start typing or open a file..."
            disabled={!setupComplete}
        ></textarea>
    </div>
</div>

<style>
    :root {
        --bg-color: #1e1e1e;
        --text-color: #d4d4d4;
        --sidebar-bg: #252526;
        --tab-bg: #2d2d2d;
    }

    .editor {
        display: flex;
        height: 100vh;
        color: var(--text-color);
        background-color: var(--bg-color);
        font-family: "Consolas", "Courier New", monospace;
    }

    .sidebar {
        width: 200px;
        background-color: var(--sidebar-bg);
        padding: 10px;
    }

    .sidebar button {
        width: 100%;
        padding: 8px;
        margin-bottom: 10px;
        background-color: #0e639c;
        color: white;
        border: none;
        cursor: pointer;
    }

    .file-list {
        margin-top: 10px;
    }

    .file {
        padding: 5px;
        cursor: pointer;
    }

    .main-content {
        flex-grow: 1;
        display: flex;
        flex-direction: column;
    }

    .tabs {
        display: flex;
        background-color: var(--tab-bg);
    }

    .tab {
        padding: 8px 15px;
        background-color: var(--bg-color);
        border-right: 1px solid #3c3c3c;
    }

    .tab.active {
        background-color: var(--bg-color);
    }

    textarea {
        flex-grow: 1;
        background-color: var(--bg-color);
        color: var(--text-color);
        border: none;
        padding: 10px;
        font-size: 14px;
        resize: none;
    }

    textarea:focus {
        outline: none;
    }
    .python-installation {
        padding: 20px;
        background-color: #2d2d2d;
        border-radius: 5px;
        margin-bottom: 20px;
    }
    .python-installation h2 {
        margin-top: 0;
    }

    .progress-bar {
        width: 100%;
        height: 20px;
        background-color: #ddd;
        border-radius: 10px;
        margin-top: 10px;
    }
    .progress {
        height: 100%;
        background-color: #4caf50;
        border-radius: 10px;
        transition: width 0.5s ease-in-out;
    }
</style>
