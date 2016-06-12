// 11 june 2016
#import "uipriv_darwin.h"

// TODO adjust all containers to handle hidden cells properly

// TODO wrap the child in a view if its align isn't fill
// maybe it's easier to do it regardless of align
@interface gridChild : NSObject
@property uiControl *c;
@property intmax_t left;
@property intmax_t top;
@property intmax_t xspan;
@property intmax_t yspan;
@property int hexpand;
@property uiAlign halign;
@property int vexpand;
@property uiAlign valign;

@property NSLayoutPriority oldHorzHuggingPri;
@property NSLayoutPriority oldVertHuggingPri;
- (NSView *)view;
@end

@interface gridView : NSView {
	uiGrid *g;
	NSMutableArray *children;
	int padded;
	uintmax_t nhexpand;
	uintmax_t nvexpand;

	NSMutableArray *edges;
	NSMutableArray *inBetweens;

	NSMutableArray *emptyCellViews;
}
- (id)initWithG:(uiGrid *)gg;
- (void)onDestroy;
- (void)removeOurConstraints;
- (void)syncEnableStates:(int)enabled;
- (CGFloat)paddingAmount;
- (void)establishOurConstraints;
- (void)append:(gridChild *)gc;
- (void)insert:(gridChild *)gc after:(uiControl *)c at:(uiAt)at;
- (int)isPadded;
- (void)setPadded:(int)p;
- (BOOL)hugsTrailing;
- (BOOL)hugsBottom;
@end

struct uiGrid {
	uiDarwinControl c;
	gridView *view;
};

@implementation gridChild

- (NSView *)view
{
	return (NSView *) uiControlHandle(self.c);
}

@end

@implementation gridView

- (id)initWithG:(uiGrid *)gg
{
	self = [super initWithFrame:NSZeroRect];
	if (self != nil) {
		self->g = gg;
		self->padded = 0;
		self->children = [NSMutableArray new];
		self->nhexpand = 0;
		self->nvexpand = 0;

		self->edges = [NSMutableArray new];
		self->inBetweens = [NSMutableArray new];

		self->emptyCellViews = [NSMutableArray new];
	}
	return self;
}

- (void)onDestroy
{
	gridChild *gc;

	[self removeOurConstraints];
	[self->edges release];
	[self->inBetweens release];

	[self->emptyCellViews release];

	for (gc in self->children) {
		uiControlSetParent(gc.c, NULL);
		uiDarwinControlSetSuperview(uiDarwinControl(gc.c), nil);
		uiControlDestroy(gc.c);
	}
	[self->children release];
}

- (void)removeOurConstraints
{
	NSView *v;

	if ([self->edges count] != 0) {
		[self removeConstraints:self->edges];
		[self->edges removeAllObjects];
	}
	if ([self->inBetweens count] != 0) {
		[self removeConstraints:self->inBetweens];
		[self->inBetweens removeAllObjects];
	}

	for (v in self->emptyCellViews)
		[v removeFromSuperview];
	[self->emptyCellViews removeAllObjects];
}

- (void)syncEnableStates:(int)enabled
{
	gridChild *gc;

	for (gc in self->children)
		uiDarwinControlSyncEnableState(uiDarwinControl(gc.c), enabled);
}

- (CGFloat)paddingAmount
{
	if (!self->padded)
		return 0.0;
	return uiDarwinPaddingAmount(NULL);
}

- (void)establishOurConstraints
{
	gridChild *gc;
	CGFloat padding;
	intmax_t xmin, ymin;
	intmax_t xmax, ymax;
	intmax_t xcount, ycount;
	BOOL first;
	int **gg;
	NSView ***gv;
	intmax_t x, y;
	int i;
	NSLayoutConstraint *c;

	[self removeOurConstraints];
	if ([self->children count] == 0)
		return;
	padding = [self paddingAmount];

	// first, figure out the minimum and maximum row and column numbers
	first = YES;
	for (gc in self->children) {
		if (first) {
			xmin = gc.left;
			ymin = gc.top;
			xmax = gc.left + gc.xspan;
			ymax = gc.top + gc.yspan;
			first = NO;
			continue;
		}
		if (xmin > gc.left)
			xmin = gc.left;
		if (ymin > gc.top)
			ymin = gc.top;
		if (xmax < (gc.left + gc.xspan))
			xmax = gc.left + gc.xspan;
		if (ymax < (gc.top + gc.yspan))
			ymax = gc.top + gc.yspan;
	}
	xcount = xmax - xmin;
	ycount = ymax - ymin;

	// now build a topological map of the grid gg[y][x]
	gg = (int **) uiAlloc(ycount * sizeof (int *), "int[][]");
	for (y = 0; y < ycount; y++) {
		gg[y] = (int *) uiAlloc(xcount * sizeof (int), "int[]");
		for (x = 0; x < xcount; x++)
			gg[y][x] = -1;		// empty
	}
	for (i = 0; i < [self->children count]; i++) {
		gc = (gridChild *) [self->children objectAtIndex:i];
		for (y = gc.top; y < gc.top + gc.yspan; y++)
			for (x = gc.left; x < gc.left + gc.xspan; x++)
				gg[y - ymin][x - xmin] = i;
	}

	// now build a topological map of the grid's views gv[y][x]
	// for any empty cell, create a dummy view
	gv = (NSView ***) uiAlloc(ycount * sizeof (NSView **), "NSView *[][]");
	for (y = 0; y < ycount; y++) {
		gv[y] = (NSView **) uiAlloc(xcount * sizeof (NSView *), "NSView *[]");
		for (x = 0; x < xcount; x++)
			if (gg[y][x] == -1) {
				gv[y][x] = [[NSView alloc] initWithFrame:NSZeroRect];
				[self addSubview:gv[y][x]];
				[self->emptyCellViews addObject:gv[y][x]];
			} else {
				gc = (gridChild *) [self->children objectAtIndex:gg[y][x]];
				gv[y][x] = [gc view];
			}
	}

	// now establish all the edge constraints
	// leading and trailing edges
	for (y = 0; y < ycount; y++) {
		c = mkConstraint(self, NSLayoutAttributeLeading,
			NSLayoutRelationEqual,
			gv[y][0], NSLayoutAttributeLeading,
			1, 0,
			@"uiGrid leading edge constraint");
		[self addConstraint:c];
		[self->edges addObject:c];
		c = mkConstraint(self, NSLayoutAttributeTrailing,
			NSLayoutRelationEqual,
			gv[y][xcount - 1], NSLayoutAttributeTrailing,
			1, 0,
			@"uiGrid trailing edge constraint");
		[self addConstraint:c];
		[self->edges addObject:c];
	}
	// top and bottom edges
	for (x = 0; x < xcount; x++) {
		c = mkConstraint(self, NSLayoutAttributeTop,
			NSLayoutRelationEqual,
			gv[0][x], NSLayoutAttributeTop,
			1, 0,
			@"uiGrid top edge constraint");
		[self addConstraint:c];
		[self->edges addObject:c];
		c = mkConstraint(self, NSLayoutAttributeBottom,
			NSLayoutRelationEqual,
			gv[ycount - 1][x], NSLayoutAttributeBottom,
			1, 0,
			@"uiGrid bottom edge constraint");
		[self addConstraint:c];
		[self->edges addObject:c];
	}

	// TODO make all expanding rows/columns the same height/width

	// and finally clean up
	for (y = 0; y < ycount; y++) {
		uiFree(gg[y]);
		uiFree(gv[y]);
	}
	uiFree(gg);
	uiFree(gv);
}

- (void)append:(gridChild *)gc
{
	NSLayoutPriority priority;
	BOOL update;
	intmax_t oldn;

	uiControlSetParent(gc.c, uiControl(self->g));
	uiDarwinControlSetSuperview(uiDarwinControl(gc.c), self);
	uiDarwinControlSyncEnableState(uiDarwinControl(gc.c), uiControlEnabledToUser(uiControl(self->g)));

	// if a control expands horizontally, it should not hug horizontally
	// otherwise, it should *forcibly* hug
	if (gc.hexpand)
		priority = NSLayoutPriorityDefaultLow;
	else
		// LONGTERM will default high work?
		priority = NSLayoutPriorityRequired;
	uiDarwinControlSetHuggingPriority(uiDarwinControl(gc.c), priority, NSLayoutConstraintOrientationHorizontal);
	// same for vertical direction
	if (gc.vexpand)
		priority = NSLayoutPriorityDefaultLow;
	else
		// LONGTERM will default high work?
		priority = NSLayoutPriorityRequired;
	uiDarwinControlSetHuggingPriority(uiDarwinControl(gc.c), priority, NSLayoutConstraintOrientationVertical);

	[self->children addObject:gc];

	[self establishOurConstraints];
	update = NO;
	if (gc.hexpand) {
		oldn = self->nhexpand;
		self->nhexpand++;
		if (oldn == 0)
			update = YES;
	}
	if (gc.vexpand) {
		oldn = self->nvexpand;
		self->nvexpand++;
		if (oldn == 0)
			update = YES;
	}
	if (update)
		uiDarwinNotifyEdgeHuggingChanged(uiDarwinControl(self->g));

	[gc release];		// we don't need the initial reference now
}

- (void)insert:(gridChild *)gc after:(uiControl *)c at:(uiAt)at
{
	gridChild *other;
	BOOL found;

	found = NO;
	for (other in self->children)
		if (other.c == c) {
			found = YES;
			break;
		}
	if (!found)
		userbug("Existing control %p is not in grid %p; you cannot add other controls next to it", c, self->g);

	switch (at) {
	case uiAtLeading:
		gc.left = other.left - gc.xspan;
		gc.top = other.top;
		break;
	case uiAtTop:
		gc.left = other.left;
		gc.top = other.top - gc.yspan;
		break;
	case uiAtTrailing:
		gc.left = other.left + other.xspan;
		gc.top = other.top;
		break;
	case uiAtBottom:
		gc.left = other.left;
		gc.top = other.top + other.yspan;
		break;
	// TODO add error checks to ALL enums
	}

	[self append:gc];
}

- (int)isPadded
{
	return self->padded;
}

- (void)setPadded:(int)p
{
	CGFloat padding;
	NSLayoutConstraint *c;

dispatch_after(
dispatch_time(DISPATCH_TIME_NOW, 3 * NSEC_PER_SEC),
dispatch_get_main_queue(),
^{ [[self window] visualizeConstraints:[self constraints]]; }
);
	self->padded = p;
	padding = [self paddingAmount];
	for (c in self->inBetweens)
		switch ([c firstAttribute]) {
		case NSLayoutAttributeLeading:
		case NSLayoutAttributeTop:
			[c setConstant:padding];
			break;
		case NSLayoutAttributeTrailing:
		case NSLayoutAttributeBottom:
			[c setConstant:-padding];
			break;
		}
}

- (BOOL)hugsTrailing
{
	// only hug if we have horizontally expanding
	return self->nhexpand != 0;
}

- (BOOL)hugsBottom
{
	// only hug if we have vertically expanding
	return self->nvexpand != 0;
}

@end

static void uiGridDestroy(uiControl *c)
{
	uiGrid *g = uiGrid(c);

	[g->view onDestroy];
	[g->view release];
	uiFreeControl(uiControl(g));
}

uiDarwinControlDefaultHandle(uiGrid, view)
uiDarwinControlDefaultParent(uiGrid, view)
uiDarwinControlDefaultSetParent(uiGrid, view)
uiDarwinControlDefaultToplevel(uiGrid, view)
uiDarwinControlDefaultVisible(uiGrid, view)
uiDarwinControlDefaultShow(uiGrid, view)
uiDarwinControlDefaultHide(uiGrid, view)
uiDarwinControlDefaultEnabled(uiGrid, view)
uiDarwinControlDefaultEnable(uiGrid, view)
uiDarwinControlDefaultDisable(uiGrid, view)

static void uiGridSyncEnableState(uiDarwinControl *c, int enabled)
{
	uiGrid *g = uiGrid(c);

	if (uiDarwinShouldStopSyncEnableState(uiDarwinControl(g), enabled))
		return;
	[g->view syncEnableStates:enabled];
}

uiDarwinControlDefaultSetSuperview(uiGrid, view)

static BOOL uiGridHugsTrailingEdge(uiDarwinControl *c)
{
	uiGrid *g = uiGrid(c);

	return [g->view hugsTrailing];
}

static BOOL uiGridHugsBottom(uiDarwinControl *c)
{
	uiGrid *g = uiGrid(c);

	return [g->view hugsBottom];
}

static void uiGridChildEdgeHuggingChanged(uiDarwinControl *c)
{
	uiGrid *g = uiGrid(c);

	[g->view establishOurConstraints];
}

uiDarwinControlDefaultHuggingPriority(uiGrid, view)
uiDarwinControlDefaultSetHuggingPriority(uiGrid, view)

static gridChild *toChild(uiControl *c, intmax_t xspan, intmax_t yspan, int hexpand, uiAlign halign, int vexpand, uiAlign valign)
{
	gridChild *gc;

	if (xspan < 0)
		userbug("You cannot have a negative xspan in a uiGrid cell.");
	if (yspan < 0)
		userbug("You cannot have a negative yspan in a uiGrid cell.");
	gc = [gridChild new];
	gc.c = c;
	gc.xspan = xspan;
	gc.yspan = yspan;
	gc.hexpand = hexpand;
	gc.halign = halign;
	gc.vexpand = vexpand;
	gc.valign = valign;
	gc.oldHorzHuggingPri = uiDarwinControlHuggingPriority(uiDarwinControl(gc.c), NSLayoutConstraintOrientationHorizontal);
	gc.oldVertHuggingPri = uiDarwinControlHuggingPriority(uiDarwinControl(gc.c), NSLayoutConstraintOrientationVertical);
	return gc;
}

void uiGridAppend(uiGrid *g, uiControl *c, intmax_t left, intmax_t top, intmax_t xspan, intmax_t yspan, int hexpand, uiAlign halign, int vexpand, uiAlign valign)
{
	gridChild *gc;

	// LONGTERM on other platforms
	// or at leat allow this and implicitly turn it into a spacer
	if (c == NULL)
		userbug("You cannot add NULL to a uiGrid.");
	gc = toChild(c, xspan, yspan, hexpand, halign, vexpand, valign);
	gc.left = left;
	gc.top = top;
	[g->view append:gc];
}

void uiGridInsertAt(uiGrid *g, uiControl *c, uiControl *existing, uiAt at, intmax_t xspan, intmax_t yspan, int hexpand, uiAlign halign, int vexpand, uiAlign valign)
{
	gridChild *gc;

	gc = toChild(c, xspan, yspan, hexpand, halign, vexpand, valign);
	[g->view insert:gc after:existing at:at];
}

int uiGridPadded(uiGrid *g)
{
	return [g->view isPadded];
}

void uiGridSetPadded(uiGrid *g, int padded)
{
	[g->view setPadded:padded];
}

uiGrid *uiNewGrid(void)
{
	uiGrid *g;

	uiDarwinNewControl(uiGrid, g);

	g->view = [[gridView alloc] initWithG:g];

	return g;
}