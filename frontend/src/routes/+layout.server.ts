import type { LayoutServerLoad } from './$types';
import jwt from 'jsonwebtoken';
import { JWT_SECRET } from '$env/static/private';

interface User {
    id: string;
    username: string;
}

export const load: LayoutServerLoad = async ({ cookies }) => {
    const token = cookies.get('auth_token');

    if (token) {
        try {
            const decoded = jwt.verify(token, JWT_SECRET) as jwt.JwtPayload;
            if (decoded && typeof decoded === 'object') {
                const user: User = {
                    id: decoded.user_id,
                    username: decoded.username
                };
                return { user };
            }
        } catch (error) {
            console.error('Failed to verify token:', error);
            // Clear the invalid cookie
            cookies.delete('auth_token', { path: '/' });
            return { user: null };
        }
    }

    return { user: null };
};