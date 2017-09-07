package com.grouter.main;

import com.badlogic.gdx.ApplicationAdapter;
import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Input;
import com.badlogic.gdx.InputProcessor;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.glutils.ShapeRenderer;

public class Main extends ApplicationAdapter implements InputProcessor {

    static final float SEGMENT_WIDTH = 5f;

    private final int segmentCount = 10;

    private boolean followingMouse = false;
    private boolean bBallEnabled = true;

    private Tentacle tentacle;
    private BouncingBall bouncingBall;
	private ShapeRenderer shapeRenderer;

	@Override
	public void create () {
        float width = Gdx.graphics.getWidth();
        float height = Gdx.graphics.getHeight();

		shapeRenderer = new ShapeRenderer();

		Gdx.input.setInputProcessor(this);

		tentacle = new Tentacle(width/2, height, segmentCount, height);
		bouncingBall = new BouncingBall(5f);
	}

	@Override
	public void render () {
		Gdx.gl.glClearColor(0.2f, 0.2f, 0.2f, 1);
		Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

		// updating
		bouncingBall.update();
        if(bBallEnabled){
            tentacle.setTarget(bouncingBall.getPosition().x, bouncingBall.getPosition().y);
        }
		tentacle.update();

		// rendering
		shapeRenderer.begin(ShapeRenderer.ShapeType.Filled);
		bouncingBall.render(shapeRenderer);
		tentacle.render(shapeRenderer);
		shapeRenderer.end();
	}
	
	@Override
	public void dispose () {
		shapeRenderer.dispose();
	}

    @Override
    public boolean keyDown(int keycode) {
	    if(keycode == Input.Keys.Q){
            followingMouse = !followingMouse;
            bBallEnabled = false;
            if (followingMouse){
                tentacle.setTarget(Gdx.input.getX(), Gdx.graphics.getHeight() - Gdx.input.getY());
            }
            return true;
        }

        if(keycode == Input.Keys.W){
            bBallEnabled = !bBallEnabled;
            followingMouse = false;
            return true;
        }

        if(keycode == Input.Keys.E){
            tentacle.setIgnoreBase(!tentacle.isIgnoreBase());
            return true;
        }

        return false;
    }

    @Override
    public boolean keyUp(int keycode) {
        return false;
    }

    @Override
    public boolean keyTyped(char character) {
        return false;
    }

    @Override
    public boolean touchDown(int screenX, int screenY, int pointer, int button) {
	    tentacle.setBase(Gdx.input.getX(), Gdx.graphics.getHeight() - Gdx.input.getY());
        return false;
    }

    @Override
    public boolean touchUp(int screenX, int screenY, int pointer, int button) {
        return false;
    }

    @Override
    public boolean touchDragged(int screenX, int screenY, int pointer) {
        return false;
    }

    @Override
    public boolean mouseMoved(int screenX, int screenY) {
        if(followingMouse){
            tentacle.setTarget(Gdx.input.getX(),Gdx.graphics.getHeight() - Gdx.input.getY());
            return true;
        }
	    return false;
    }

    @Override
    public boolean scrolled(int amount) {
        return false;
    }
}
