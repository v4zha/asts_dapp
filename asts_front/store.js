import { configureStore, getDefaultMiddleware } from '@reduxjs/toolkit'
import authReducer from './Slices/loginSlice'
import studentReducer from './Slices/studentSlice'
import eventReducer from './Slices/eventSlice'

export const store = configureStore({
  reducer: {
    auth: authReducer,
    student: studentReducer,
    event: eventReducer
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: false,
    }),
});
