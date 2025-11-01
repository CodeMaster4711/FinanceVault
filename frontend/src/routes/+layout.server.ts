import type { LayoutServerLoad } from './$types';
import jwt from 'jsonwebtoken';


interface User {
    id: string;
    username: string;
}

export const load: LayoutServerLoad = async ({ cookies }) => {
    const token = cookies.get('auth_token');

    if (token) {
        try {
            console.log('JWT_SECRET (from process.env):', process.env.JWT_SECRET ? 'DEFINED' : 'UNDEFINED');
            console.log('Token received:', token ? 'DEFINED' : 'UNDEFINED', token ? token.substring(0, 10) + '...' : '');
            if (!process.env.JWT_SECRET) {
                console.error('JWT_SECRET is not defined in +layout.server.ts');
                cookies.delete('auth_token', { path: '/' });
                return { user: null };
            }
            const decoded = jwt.verify(token, process.env.JWT_SECRET) as jwt.JwtPayload;
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