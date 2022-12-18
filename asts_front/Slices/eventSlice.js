import { createSlice } from '@reduxjs/toolkit'

export const eventSlice = createSlice({
  name: 'event',
  initialState: {
    event_name: "",
    event_id: ""
  },
  reducers: {
    getEvents: (state, action) => {
      state.event_name = action.payload.event_name
      state.event_id = action.payload.event_id
    }
  }
})

export const { getEvents } = eventSlice.actions
export default eventSlice.reducer
