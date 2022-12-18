// SPDX-License-Identifier: MIT
pragma abicoder v2;
pragma solidity >=0.7.0 <0.9.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

import {Student} from "./StdTokenSys.sol";

contract ASTEvent is ERC721 {
    address public owner;
    uint256 public tokenCounter;

    event EventAdded(uint256 tkCounter);
    event EventApproved(uint256 tkCounter);
    event EventRejected(uint256 tkCounter);
    event EventEnded(uint256 tkCounter);
    event EventLive(uint256 tkCounter);

    enum EventState {
        Upcoming,
        Live,
        Finished
    }
    Student std;
    struct EventDetails {
        string name;
        string metadata;
        address organizer;
        address winner;
        EventState state;
        bool approval;
        Student.TokenKind kind;
        uint32 tokenAmt;
        uint32 stakeAmt;
    }
    mapping(uint256 => EventDetails) public astEvents;
    modifier isStudent(address stdAddr) {
        require(
            std.getStdStatus(stdAddr),
            "This operation can only be perfomed by registered Students"
        );
        _;
    }
    modifier hasToken(
        address stdAddr,
        Student.TokenKind kind,
        uint32 stakeAmt
    ) {
        require(
            stakeAmt <= std.getTokenAmt(stdAddr, kind) && stakeAmt >= 1,
            "The Student doesn't have the required no of tokens "
        );
        _;
    }

    modifier isOrganizer(uint256 tokenId) {
        require(
            msg.sender == astEvents[tokenId].organizer,
            "This operation can only be performed by event organizer"
        );
        _;
    }
    modifier onlyAdmin() {
        require(
            msg.sender == owner,
            "This operation cann only be performed by administrator"
        );
        _;
    }

    modifier isApproved(uint256 tokenId) {
        require(
            astEvents[tokenId].approval,
            "This operation cannot be performed as Event is not approved"
        );
        _;
    }

    constructor(address stdContractAddress) ERC721("STEvents", "STE") {
        std = Student(stdContractAddress);
        owner = msg.sender;
        tokenCounter = 0;
    }

    function createASTEvent(
        string memory name,
        string memory metadata,
        address organizer,
        Student.TokenKind kind,
        uint32 tokenAmt,
        uint32 stakeAmt
    ) public isStudent(organizer) hasToken(organizer, kind, stakeAmt) {
        EventDetails memory stdEvent = EventDetails({
            name: name,
            metadata: metadata,
            organizer: organizer,
            state: EventState.Upcoming,
            winner: address(0),
            tokenAmt: tokenAmt,
            stakeAmt: stakeAmt,
            kind: kind,
            approval: false
        });
        tokenCounter += 1;
        _mint(organizer, tokenCounter);
        _setEventDetails(stdEvent, tokenCounter);
        emit EventAdded(tokenCounter);
    }

    function _setEventDetails(EventDetails memory stdEvent, uint256 tokenId)
        internal
    {
        astEvents[tokenId] = stdEvent;
    }

    function endASTEvent(address winStd, uint256 tokenId)
        external
        isOrganizer(tokenId)
        isApproved(tokenId)
    {
        astEvents[tokenId].state = EventState.Finished;
        astEvents[tokenId].winner = winStd;
        Student.TokenKind kind = astEvents[tokenId].kind;
        uint32 amt = astEvents[tokenId].tokenAmt;
        std.addToken(winStd, amt, kind, std.getOwner());
        emit EventEnded(tokenId);
    }

    function setStdEventLive(uint256 tokenId)
        external
        isOrganizer(tokenId)
        isApproved(tokenId)
    {
        astEvents[tokenId].state = EventState.Live;
        emit EventLive(tokenId);
    }

    function approveEvent(uint256 tokenId) external onlyAdmin returns (bool) {
        astEvents[tokenId].approval = true;
        emit EventApproved(tokenId);
        return true;
    }

    function getASTEventData(uint256 tokenId)
        external
        view
        returns (EventDetails memory)
    {
        return astEvents[tokenId];
    }

    function submitEvent(
        string memory eventName,
        string memory metadata,
        Student.TokenKind kind,
        uint32 tokenAmt,
        uint32 stakeAmt
    ) external {
        createASTEvent(
            eventName,
            metadata,
            msg.sender,
            kind,
            tokenAmt,
            stakeAmt
        );
    }
}
