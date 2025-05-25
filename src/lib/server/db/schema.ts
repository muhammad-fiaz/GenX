import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const users = sqliteTable('user', {
	id: text('id').primaryKey(),
	githubId: integer('github_id').unique(),
	username: text('username'),
	avatarUrl: text('avatar_url'),
	email: text('email'), // Add email field
	name: text('name'), // Add name field
	createdAt: integer('created_at', { mode: 'timestamp' }),
	expiresAt: integer('expires_at', { mode: 'timestamp' }) // For expiry
});
