#include "uipriv_windows.hpp"
#include <array>

#define MAX_STATUSBAR 256 /* maximum amount of status bar parts */

struct uiStatusBar
{
    int partCount;
    HWND hwnd;
    std::vector<const char*> parts;
};

uiWindowsControlAllDefaults(uiStatusBar)

static void uiStatusBarMinimumSize(uiWindowsControl* c, int* width, int* height)
{
    // TODO: implement
    *width = 5;
    *height = 10;
}

static void CalculatePart(uiStatusBar* b, int* outParts[], int* outPartsSize)
{
    RECT rcClient;
    int i, nWidth;
    HLOCAL hLoc;
    PINT paParts;
    size_t partsSize = b->parts.size();

    uiWindowsControl* wc = uiWindowsControl(b);
    GetClientRect((HWND) wc->parent->Handle(wc->parent), &rcClient);
    
    hLoc = LocalAlloc(LHND, sizeof(int) * partsSize);
    paParts = (PINT)LocalLock(hLoc);

    nWidth = rcClient.right / partsSize;
    int rightEdge = nWidth;
    for (i = 0; i < partsSize; i++)
    {
        paParts[i] = rightEdge;
        rightEdge += nWidth;
    }
    int ffsize = sizeof(paParts) / sizeof(int);
    (*outPartsSize) = ffsize;

    memcpy(outParts, paParts, ffsize);
    LocalUnlock(hLoc);
    LocalFree(hLoc);
}

uiStatusBar* uiNewStatusBar(int partCount, const char* partText[])
{
    BYTE op;
    uiStatusBar* b;

    uiWindowsNewControl(uiStatusBar, b);
    b->partCount = partCount;
    b->parts.assign(partText, partText + partCount);

    b->hwnd = uiWindowsEnsureCreateControlHWND(
        0,
        STATUSCLASSNAME,
        nullptr,
        SBARS_SIZEGRIP,
        hInstance,
        nullptr,
        TRUE
    );

    int** paParts;
    int iPartsSize;
    CalculatePart(b, paParts, &iPartsSize);
    SendMessageW(b->hwnd, SB_SETPARTS, (WPARAM) partCount, 
                 (LPARAM) paParts);

    for (int i = 0; i < iPartsSize; i++)
    {
        op = (op & 0x00ff) | (SBT_NOTABPARSING << 8);
        op = (op & 0xff00) | i;
        SendMessageW(b->hwnd, SB_SETTEXT,
            (WPARAM)MAKEWORD(op, 0),
            (LPARAM)paParts[i]);
        op = 0;
    }
    return b;
}

void uiStatusBarAppend(uiStatusBar* b, const char* nPart)
{
    if (b->parts.size() >= MAX_STATUSBAR)
        return;
    
    b->parts.push_back(nPart);
    int** paParts;
    int paCount;

    CalculatePart(b, paParts, &paCount);
    SendMessageW(b->hwnd, SB_SETPARTS, 
        (WPARAM)b->parts.size(), 
        (LPARAM)paParts);
}

void uiStatusBarRemove(uiStatusBar* b, unsigned char position)
{
    if (b->parts.size() >= MAX_STATUSBAR)
        return;

    b->parts.erase(b->parts.begin() + position);
    int** paParts;
    int paCount;

    CalculatePart(b, paParts, &paCount);
    SendMessageW(b->hwnd, SB_SETPARTS,
        (WPARAM)b->parts.size(),
        (LPARAM)paParts);
}
