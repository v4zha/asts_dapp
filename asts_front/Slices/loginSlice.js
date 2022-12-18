import { createSlice } from '@reduxjs/toolkit'

export const authSlice = createSlice({
  name: 'auth',
  initialState: {
   ctx: null,
  },
  reducers: {
    studentSignin: (state, action) => {
      state.ctx = action.payload.ctx;
    },
    adminSignin: (state, action) => {
      state.ctx = action.payload.ctx;
    },
  },
})

export const { studentSignin, adminSignin } = authSlice.actions

export default authSlice.reducer
