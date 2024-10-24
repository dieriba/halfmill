<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import { enhance } from '$app/forms';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;
	let editorContainer: HTMLElement;
	let selectedTheme = 'vs-dark';
	let selectedLanguage = 'javascript';
	let editorContent = '';
	onMount(async () => {
		monaco = (await import('../lib/monaco_editor')).default;
		editor = monaco.editor.create(editorContainer, {
			theme: selectedTheme,
			language: selectedLanguage,
			automaticLayout: true,
			minimap: { enabled: true },
			fontSize: 14,
			lineNumbers: 'on',
			roundedSelection: true,
			scrollBeyondLastLine: false,
			readOnly: false
		});

		const model = monaco.editor.createModel('// Start coding here...', selectedLanguage);

		editor.onDidChangeModelContent(() => {
			editorContent = editor.getValue();
		});

		editor.setModel(model);
	});

	onDestroy(() => {
		monaco?.editor.getModels().forEach((model) => model.dispose());
		editor?.dispose();
	});

	const updateLanguage = (event: Event) => {
		selectedLanguage = (event.target as HTMLSelectElement).value;
		monaco?.editor.setModelLanguage(editor.getModel()!, selectedLanguage);
	};
</script>

<div class="h-5/6 w-5/6 bg-gray-50 p-4 dark:bg-gray-900">
	<div class="flex h-full flex-col p-4">
		<div class="mb-4 flex items-center justify-between">
			<div class="flex items-center gap-4">
				<form
					method="POST"
					action="?/script"
					use:enhance={() => {
						return async ({ update }) => {
							await update({ reset: false });
						};
					}}
				>
					<select
						class="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
						on:change={updateLanguage}
					>
						<option value="javascript">JavaScript</option>
					</select>
					<button
						formaction="?/script"
						class="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-green-600 hover:bg-green-50 focus:outline-none focus:ring-2 focus:ring-green-500 disabled:opacity-50 dark:border-gray-600 dark:bg-gray-700 dark:text-green-400 dark:hover:bg-gray-600"
					>
						Execute Script
					</button>
					<button
						formaction="?/logout"
						class="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-red-600 hover:bg-red-50 focus:outline-none focus:ring-2 focus:ring-red-500 disabled:opacity-50 dark:border-gray-600 dark:bg-gray-700 dark:text-red-400 dark:hover:bg-gray-600"
					>
						Logout
					</button>
					<input type="hidden" name="language" value={selectedLanguage} />
					<input type="hidden" name="code" value={editorContent ?? ''} />
				</form>
			</div>
		</div>
		<div
			class="h-full w-full overflow-hidden rounded-lg border border-gray-200 dark:border-gray-700"
			bind:this={editorContainer}
		></div>
	</div>
</div>

<style>
</style>
