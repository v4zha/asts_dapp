import { createSlice } from "@reduxjs/toolkit";

export const studentSlice = createSlice({
    name : "student",
    initialState : {
        name : "",
        address : '',
        department : "",
        admn_no : "",
        tokens : {
            academic : 0,
            social : 0,
            sports : 0
        }
    },
    reducers : {
        getStudent : (state, action) => {
            state.name = action.payload.name
            state.address = action.payload.address
            state.department = action.payload.department
            state.admn_no = action.payload.admn_no
            state.tokens = action.payload.tokens

        }
    }
})

export const { getStudent } = studentSlice.actions

export default studentSlice.reducer
