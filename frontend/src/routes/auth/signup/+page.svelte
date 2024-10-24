<script lang="ts">
	import { Section, Register } from 'flowbite-svelte-blocks';
	import { Button, Label, Input, Helper, Spinner } from 'flowbite-svelte';
	import { enhance } from '$app/forms';

	let isLoading = false;

	/** @type {import('./$types').ActionData} */
	export let form;
</script>

<Section name="register">
	<Register>
		<div class="w-[500px] space-y-4 p-6 sm:p-8 md:space-y-6">
			{#if form}
				{#if form.success && form.message}
					{#each form.message as message}
						<Helper class="mt-2" color="green">
							<span class="block font-medium">{message}</span>
						</Helper>
					{/each}
				{:else if form.success == false && form.message}
					{#each form.message as message}
						<Helper class="mt-2" color="red">
							<span class="block font-medium">{message}</span>
						</Helper>
					{/each}
				{/if}
			{/if}

			<form
				method="POST"
				class="flex flex-col space-y-6"
				action="?/register"
				use:enhance={() => {
					isLoading = true;
					return async ({ update }) => {
						await update({ reset: false });
						isLoading = false;
					};
				}}
			>
				<div>
					<Label class="space-y-2">
						<span>Username</span>
						<Input type="text" name="username" placeholder="dieriba" />
					</Label>
					{#if form?.errors?.username}
						<Helper class="mt-2" color="red">
							{#each form?.errors?.username as message}
								<span class="font-medium">{message}</span>
							{/each}
						</Helper>
					{/if}
				</div>
				<div>
					<Label class="space-y-2">
						<span>Password</span>
						<Input type="password" name="password" placeholder="••••••••" />
					</Label>
					{#if form?.errors?.password}
						<Helper class="mt-2" color="red">
							{#each form?.errors?.password as message}
								<span class="font-medium">{message}</span>
							{/each}
						</Helper>
					{/if}
				</div>
				<div>
					<Label class="space-y-2">
						<span>Confirm Password</span>
						<Input type="password" name="confirmPassword" placeholder="••••••••" />
					</Label>
					{#if form?.errors?.confirmPassword}
						<Helper class="mt-2" color="red">
							{#each form?.errors?.confirmPassword as message}
								<span class="font-medium">{message}</span>
							{/each}
						</Helper>
					{/if}
				</div>
				<Button type="submit" class="w-full1"
					>{#if isLoading == false}
						Sign up
					{:else}
						<Spinner />
					{/if}
				</Button>
				<p class="text-sm font-light text-gray-500 dark:text-gray-400">
					Alreay have an account? <a
						data-sveltekit-reload
						href="/auth/signin"
						class="font-medium text-primary-600 hover:underline dark:text-primary-500">Sign in</a
					>
				</p>
			</form>
		</div>
	</Register>
</Section>
